use std::fmt::Debug;
use std::path::Path;

use crate::error::Result;
use crate::ChildEntity;
use crate::{Storable, XvcStore};

/// RMNStore is M-N RelationStore, where we store arbitrary relationships between two entities. It
/// doesn't have any semantics except binding two entities together.
#[derive(Debug, Clone)]
pub struct RMNStore<T, U>
where
    T: Storable,
    U: Storable,
{
    /// The store for M side of the relationships
    pub left: XvcStore<T>,
    /// The store for N side of the relationships
    pub right: XvcStore<U>,
    /// Parent-child relationships from left to right
    pub left_to_right: XvcStore<ChildEntity<T, U>>,
    /// Parent-child relationships from right to left
    pub right_to_left: XvcStore<ChildEntity<U, T>>,
}

impl<T, U> RMNStore<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Loads store from the directory under store root named after types
    pub fn load_rmnstore(store_root: &Path) -> Result<RMNStore<T, U>> {
        let left = XvcStore::<T>::load_store(store_root)?;
        let right = XvcStore::<U>::load_store(store_root)?;
        let left_to_right = XvcStore::<ChildEntity<T, U>>::load_store(&store_root)?;
        let right_to_left = XvcStore::<ChildEntity<U, T>>::load_store(&store_root)?;
        Ok(RMNStore {
            left,
            right,
            left_to_right,
            right_to_left,
        })
    }

    /// Saves the store to directory created by store root and type names
    pub fn save_rmnstore(store: &Self, store_root: &Path) -> Result<()> {
        store.left.save(store_root)?;
        store.right.save(store_root)?;
        store.left_to_right.save(store_root)?;
        store.right_to_left.save(store_root)
    }
}
