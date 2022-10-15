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
    pub fn new() -> Self {
        Self {
            vec: Vec::<(XvcEntity, T)>::new(),
            current: EventLog::new(),
            previous: EventLog::new(),
        }
    }

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
                Event::Remove { entity } => map.remove(&entity),
            };
        }

        let vec = Vec::from_iter(map.into_iter());

        vec
    }

    pub fn from_dir(dir: &Path) -> Result<Self> {
        let previous = EventLog::<T>::from_dir(dir)?;
        let vec = Self::build_vec(&previous);

        Ok(Self {
            vec,
            previous,
            current: EventLog::new(),
        })
    }

    pub fn to_dir(&self, dir: &Path) -> Result<()> {
        self.current.to_dir(dir)
    }

    pub fn values_of(&self, parent: &XvcEntity) -> Vec<T> {
        self.vec
            .iter()
            .filter_map(|(e, v)| if *e == *parent { Some(v.clone()) } else { None })
            .collect()
    }

    pub fn insert(&mut self, entity: XvcEntity, value: T) {
        self.current.add(entity, value.clone());
        self.vec.push((entity, value))
    }

    pub fn to_store(&self) -> Result<XvcStore<T>> {
        let mut store = XvcStore::<T>::new();
        self.vec.iter().for_each(|(e, v)| {
            store.insert(*e, v.clone());
        });
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

// impl<T> From<Vec<(XvcEntity, T)>> for VStore<T>
// where
//     T: Serialize + for<'lt> Deserialize<'lt> + Clone,
// {
//     fn from(vec: Vec<(XvcEntity, T)>) -> Self {
//         Self { vec }
//     }
// }
//

// impl<T> From<BStore<T>> for VStore<T>
// where
//     T: Storable,
// {
//     fn from(store: BStore<T>) -> Self {
//         match store.to_vstore() {
//             Ok(vs) => vs,
//             Err(_) => {
//                 Error::StoreConversionError.error();
//                 Self::new()
//             }
//         }
//     }
// }
//
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

    pub fn load_vstore(store_root: &Path) -> Result<VStore<T>> {
        let dir = Self::vstore_path(store_root);
        VStore::<T>::from_dir(&dir)
    }

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
        store.insert(0.into(), "0".into());
        store.insert(1.into(), "1".into());
        assert_eq!(store.len(), 2);

        assert_eq!(store.vec.pop().unwrap().1, "1".to_string());
        assert_eq!(store.vec.pop().unwrap().1, "0".to_string());
        Ok(())
    }
}
