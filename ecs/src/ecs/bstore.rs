use super::event::Event;
use super::event::EventLog;
use super::*;
use crate::error::{Error, Result};
use crate::Storable;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use super::hstore::HStore;
use super::vstore::VStore;

use std::ops::Deref;
use std::ops::Index;

/// This is the usual BTreeMap to attach XvcEntity to Components
#[deprecated]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub struct BStore<T>
where
    T: Storable,
{
    map: BTreeMap<XvcEntity, T>,
    previous: EventLog<T>,
    current: EventLog<T>,
}

impl<T> Deref for BStore<T>
where
    T: Storable,
{
    type Target = BTreeMap<XvcEntity, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> Index<&XvcEntity> for BStore<T>
where
    T: Storable,
{
    type Output = T;

    fn index(&self, entity: &XvcEntity) -> &Self::Output {
        self.map.index(entity)
    }
}

impl<T> Default for BStore<T>
where
    T: Storable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BStore<T>
where
    T: Storable,
{
    pub fn new() -> Self {
        Self {
            map: BTreeMap::<XvcEntity, T>::new(),
            previous: EventLog::<T>::new(),
            current: EventLog::<T>::new(),
        }
    }

    /// Calls the inner map's insert
    pub fn insert(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        self.current.add(entity, value.clone());
        self.map.insert(entity, value)
    }

    pub fn update(&mut self, entity: XvcEntity, value: T) -> Option<T> {
        if self.map.contains_key(&entity) {
            self.remove(entity);
        }
        self.insert(entity, value)
    }

    pub fn remove(&mut self, entity: XvcEntity) -> Option<T> {
        self.current.remove(entity);
        self.map.remove(&entity)
    }

    pub fn append(&mut self, other: &Self) -> Result<()> {
        other.iter().for_each(|(e, v)| {
            self.insert(*e, v.clone());
        });
        Ok(())
    }

    // pub fn to_json(&self) -> Result<JsonValue> {
    //     serde_json::to_value(self).map_err(|e| Error::JsonError { source: e }.warn())
    // }
    //
    // pub fn from_json(json_str: &str) -> Result<BStore<T>> {
    //     serde_json::from_str(json_str).map_err(|e| Error::JsonError { source: e }.warn())
    // }
    //
    // pub fn to_msgpack(&self) -> Result<Vec<u8>> {
    //     let mut value = Vec::new();
    //     match self.serialize(&mut rmp_serde::Serializer::new(&mut value)) {
    //         Ok(_) => Ok(value),
    //         Err(source) => Err(Error::MsgPackEncodeError { source }.warn()),
    //     }
    // }
    //
    // pub fn from_msgpack(msgpack_val: &[u8]) -> Result<BStore<T>> {
    //     let cursor = io::Cursor::new(msgpack_val);
    //     let mut deser = rmp_serde::decode::Deserializer::new(cursor);
    //     let val = Deserialize::deserialize(&mut deser);
    //     match val {
    //         Ok(md) => Ok(md),
    //         Err(source) => Err(Error::MsgPackDecodeError { source }.warn()),
    //     }
    // }
    //

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

    pub fn from_dir(dir: &Path) -> Result<Self> {
        let previous = EventLog::<T>::from_dir(dir)?;
        watch!(previous);
        let map = Self::build_map(&previous);
        watch!(map);

        Ok(Self {
            map,
            previous,
            current: EventLog::new(),
        })
    }

    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        self.current.to_dir(dir)
    }

    fn from_file(path: &Path) -> Result<BStore<T>> {
        let previous = EventLog::<T>::from_file(path)?;
        let map = Self::build_map(&previous);

        Ok(Self {
            map,
            previous,
            current: EventLog::new(),
        })
    }

    pub fn to_vstore(&self) -> Result<VStore<T>> {
        let mut vstore = VStore::<T>::with_capacity(self.len());
        self.iter().for_each(|(k, v)| {
            vstore.insert(*k, v.clone());
        });
        Ok(vstore)
    }

    pub fn to_hstore(&self) -> Result<HStore<T>> {
        let mut hstore = HStore::<T>::with_capacity(self.len());

        self.iter().for_each(|(k, v)| {
            hstore.map.insert(*k, v.clone());
        });
        Ok(hstore)
    }

    pub fn from_func<F>(gen: &XvcEntityGenerator, func: F) -> Result<Self>
    where
        F: Fn() -> Result<Option<T>>,
    {
        let mut map = BTreeMap::<XvcEntity, T>::new();
        let mut current = EventLog::<T>::new();

        loop {
            let value = match func() {
                Ok(Some(v)) => v,
                Ok(None) => break,
                Err(err) => return Err(err),
            };
            let entity = gen.next_element();
            map.insert(entity, value.clone());
            current.add(entity, value);
        }
        Ok(Self {
            map,
            current,
            previous: EventLog::new(),
        })
    }

    pub fn from_iter<I>(gen: &XvcEntityGenerator, iter: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
    {
        let mut map = BTreeMap::<XvcEntity, T>::new();
        let mut current = EventLog::<T>::new();
        for value in iter {
            let entity = gen.next_element();
            map.insert(entity, value.clone());
            current.add(entity, value);
        }
        Ok(Self {
            map,
            current,
            previous: EventLog::new(),
        })
    }

    pub fn to_hset(&self) -> HashSet<T>
    where
        T: std::fmt::Debug + Eq + std::hash::Hash,
    {
        let mut set: HashSet<T> = HashSet::<T>::with_capacity(self.len());
        for (e, v) in self.iter() {
            let vn = v.clone();
            if !set.insert(vn) {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        set
    }

    pub fn to_bset(&self) -> BTreeSet<T>
    where
        T: std::fmt::Debug + std::cmp::Ord,
    {
        let mut set = BTreeSet::<T>::new();
        for (e, v) in self.iter() {
            if !set.insert(v.clone()) {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        set
    }

    pub fn inverted_hmap(&self) -> HashMap<T, XvcEntity>
    where
        T: std::fmt::Debug + Eq + std::hash::Hash,
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
    pub fn inverted_bmap(&self) -> BTreeMap<T, XvcEntity>
    where
        T: std::fmt::Debug + std::cmp::Ord,
    {
        let mut imap = BTreeMap::<T, XvcEntity>::new();
        for (e, v) in self.iter() {
            let ires = imap.insert(v.clone(), *e);
            if ires != None {
                debug!("Duplicate value in store: ({:?}, {:?})", e, v);
            }
        }
        imap
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn subset<I>(&self, keys: I) -> Result<Self>
    where
        I: Iterator<Item = XvcEntity>,
    {
        let mut store = BStore::<T>::new();
        for e in keys {
            if let Some(v) = self.map.get(&e) {
                store.insert(e, v.clone());
            } else {
                Error::CannotFindKeyInStore { key: e.0 }.warn();
            }
        }
        Ok(store)
    }

    pub fn filter<F>(&self, predicate: F) -> Vec<XvcEntity>
    where
        F: Fn(&XvcEntity, &T) -> bool,
    {
        let mut vec = Vec::<XvcEntity>::new();
        for (e, v) in self.map.iter() {
            if predicate(e, v) {
                vec.push(*e);
            }
        }

        vec
    }

    /// Searches the map values linearly and returns the entity if found
    /// For heavy usage, it may be more logical to use [Self::inverted_bmap] and
    /// [Self::inverted_hmap] functions to get an index.
    pub fn entity_by_value(&self, value: &T) -> Option<XvcEntity>
    where
        T: PartialEq,
    {
        self.map
            .iter()
            .find(|(_, v)| **v == *value)
            .map(|(e, _)| *e)
    }
}

impl<T> BStore<T>
where
    T: Storable,
{
    fn bstore_path(store_root: &Path) -> PathBuf {
        store_root.join(format!("{}-bstore", <T as Storable>::type_description()))
    }

    pub fn load_bstore(store_root: &Path) -> Result<BStore<T>> {
        let dir = Self::bstore_path(store_root);
        watch!(&dir);
        Self::from_dir(&dir)
    }

    pub fn save_bstore(store: &BStore<T>, store_root: &Path) -> Result<()> {
        let dir = Self::bstore_path(store_root);
        store.to_dir(&dir)
    }

    pub fn save(&self, store_root: &Path) -> Result<()> {
        Self::save_bstore(self, store_root)
    }
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;

    use super::*;

    #[test]
    fn new() -> Result<()> {
        let mut store = BStore::<String>::new();
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

        let mut store = BStore::<String>::new();

        store.insert(0.into(), "0".into());
        store.insert(1.into(), "1".into());
        store.insert(2.into(), "2".into());

        store.to_dir(&dir)?;

        let reincarnation = BStore::<String>::from_dir(&dir)?;

        assert!(store.len() == reincarnation.len());

        Ok(())
    }
}
