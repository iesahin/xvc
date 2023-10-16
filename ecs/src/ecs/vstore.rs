//! [VStore] and its implementation allows [Storable] types to be processed with [std::vector].
//! It's better suited when duplicate elements with the same `XvcEntity` may occur.
use super::event::{Event, EventLog};
use super::hstore::HStore;
use crate::error::{Error, Result};
use crate::XvcEntity;
use crate::{Storable, XvcStore};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// This is a tuple-vector for (XvcEntity, T) to enable faster serial access
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub struct VStore<T>
where
    T: Storable,
{
    vec: Vec<(XvcEntity, T)>,
    previous: EventLog<T>,
    current: EventLog<T>,
}

impl<T> Default for VStore<T>
where
    T: Storable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VStore<T>
where
    T: Storable,
{
    /// Create an empty [VStore] with empty logs and vector.
    pub fn new() -> Self {
        Self {
            vec: Vec::<(XvcEntity, T)>::new(),
            current: EventLog::new(),
            previous: EventLog::new(),
        }
    }

    /// Create the vector with the given capacity to reduce later memory expansions.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::<(XvcEntity, T)>::with_capacity(capacity),
            // TODO: The following may also use with_capacity
            current: EventLog::new(),
            previous: EventLog::new(),
        }
    }

    fn build_vec(events: &EventLog<T>) -> Vec<(XvcEntity, T)> {
        let mut map = HashMap::<XvcEntity, T>::new();

        for event in events.iter() {
            match event {
                Event::Add { entity, value } => map.insert(*entity, value.clone()),
                Event::Remove { entity } => map.remove(entity),
            };
        }

        Vec::from_iter(map)
    }

    /// Loads the store from the given `dir`.
    /// It loads the [event log][EventLog]  and builds ([`XvcEntity`], `T`) by replaying it.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let previous = EventLog::<T>::from_dir(dir)?;
        let vec = Self::build_vec(&previous);

        Ok(Self {
            vec,
            previous,
            current: EventLog::new(),
        })
    }

    /// Saves the event log to a directory.
    /// It only saves the events added after the last load.
    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        self.current.to_dir(dir)
    }

    /// Get all elements associated with an [`XvcEntity`].
    ///
    /// Conventionally each entity is associated with a single component type, but in some cases an
    /// entity (e.g. a triangle) may be associated with multiple components. (e.g. points.)
    /// Nevertheless, in such cases, we recommend to use [R1NStore] to mark the relationship
    /// clearly.
    pub fn values_of(&self, parent: &XvcEntity) -> Vec<T> {
        self.vec
            .iter()
            .filter_map(|(e, v)| if *e == *parent { Some(v.clone()) } else { None })
            .collect()
    }

    /// Insert an entity to the store.
    /// It also inserts an event to the log.
    pub fn insert(&mut self, entity: XvcEntity, value: T) {
        self.current.add(entity, value.clone());
        self.vec.push((entity, value))
    }

    /// Convert to an [XvcStore].
    /// Uses [XvcStore::from_event_logs] to create a new store self.previous and self.current
    /// clones.
    pub fn to_store(&self) -> Result<XvcStore<T>> {
        let store = XvcStore::from_event_logs(self.previous.clone(), self.current.clone());
        Ok(store)
    }
}

impl<T> Deref for VStore<T>
where
    T: Storable,
{
    type Target = Vec<(XvcEntity, T)>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> From<HStore<T>> for VStore<T>
where
    T: Storable,
{
    fn from(store: HStore<T>) -> Self {
        match store.to_vstore() {
            Ok(vs) => vs,
            Err(_) => {
                Error::StoreConversionError.error();
                Self::new()
            }
        }
    }
}

impl<T> FromIterator<(XvcEntity, T)> for VStore<T>
where
    T: Storable,
{
    fn from_iter<I: IntoIterator<Item = (XvcEntity, T)>>(iter: I) -> Self {
        let mut s = Self::new();
        for (e, v) in iter {
            s.insert(e, v.clone());
        }
        s
    }
}

impl<T> VStore<T>
where
    T: Storable,
{
    fn vstore_path(store_root: &Path) -> PathBuf {
        store_root.join(format!("{}-vstore", <T as Storable>::type_description()))
    }

    /// Loads a store by appending `T`'s type name (see [Storable::type_description]) to
    /// `store_root.`
    pub fn load_vstore(store_root: &Path) -> Result<VStore<T>> {
        let dir = Self::vstore_path(store_root);
        VStore::<T>::from_dir(&dir)
    }

    /// Saves a store by appending `T`'s type name (see [Storable::type_description]) to
    /// `store_root.`
    pub fn save_vstore(&self, store_root: &Path) -> Result<()> {
        let dir = Self::vstore_path(store_root);
        self.to_dir(&dir)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() -> Result<()> {
        let mut store = VStore::<String>::new();
        store.insert((0, 12398012938).into(), "0".into());
        store.insert((1, 12398012938).into(), "1".into());
        assert_eq!(store.len(), 2);

        assert_eq!(store.vec.pop().unwrap().1, "1".to_string());
        assert_eq!(store.vec.pop().unwrap().1, "0".to_string());
        Ok(())
    }
}
