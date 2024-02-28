//! Represents 1-N (parent-child) relationships.
//! Each child has a one parent but each parent can have multiple children.
use super::XvcEntity;
use crate::error::{Error, Result};
use crate::{HStore, Storable, XvcStore};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::Path;

/// Wrapper around XvcEntity that represents a parent-child relationship.
///
/// The key for the XvcStore that keeps the relatinship is the child entity, as there are many
/// children for a parent. XvcStore doesn't allow duplicate keys so XvcStore<ChildEntity<T, U>> is
/// a 1-N relationship between T and U. There are many T (children) for a U (parent).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChildEntity<T: Storable, U: Storable>(XvcEntity, PhantomData<T>, PhantomData<U>);

impl<T: Storable, U: Storable> Storable for ChildEntity<T, U> {
    fn type_description() -> String {
        format!(
            "{}-{}-r1n",
            <T as Storable>::type_description(),
            <U as Storable>::type_description()
        )
    }
}

impl<T: Storable, U: Storable> From<XvcEntity> for ChildEntity<T, U> {
    fn from(xe: XvcEntity) -> Self {
        Self(xe, PhantomData, PhantomData)
    }
}

impl<T: Storable, U: Storable> From<ChildEntity<T, U>> for XvcEntity {
    fn from(r1ne: ChildEntity<T, U>) -> Self {
        r1ne.0
    }
}

impl<T: Storable, U: Storable> Deref for ChildEntity<T, U> {
    type Target = XvcEntity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// RNStore is 1-N RelationStore, where we store one-to-many relationships between two entities.
/// It doesn't have any semantics except binding two entities together.
#[derive(Debug, Clone)]
pub struct R1NStore<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Keeps the parent type
    pub parents: XvcStore<T>,
    /// Keeps the child type
    pub children: XvcStore<U>,
    /// Keeps the relationships between child entities and parent entities
    /// The key for the child_parents is the child entity, as there are many children for a parent.
    pub child_parents: XvcStore<ChildEntity<U, T>>,
}

impl<T, U> R1NStore<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Insert a child component to a parent parent_component.
    /// It checks whether the parent is equal to the given, and updates if there is a change.
    pub fn insert(
        &mut self,
        parent_entity: XvcEntity,
        parent_component: T,
        child_entity: XvcEntity,
        child_component: U,
    ) -> Option<XvcEntity> {
        match self.parents.get(&parent_entity) {
            None => {
                self.parents.insert(parent_entity, parent_component);
            }
            Some(value) => {
                if *value != parent_component {
                    self.parents.update(parent_entity, parent_component);
                }
            }
        }

        self.children.insert(child_entity, child_component);
        // CAUTION: The order is reversed!
        self.child_parents
            .insert(child_entity, parent_entity.into())
            .map(XvcEntity::from)
    }

    /// get the store of related entities.
    pub fn children_of(&self, parent_entity: &XvcEntity) -> Result<HStore<U>> {
        let related_entities = self.child_parents.iter().filter_map(|(child, parent)| {
            if *parent == (*parent_entity).into() {
                Some(*child)
            } else {
                None
            }
        });
        self.children.subset(related_entities)
    }

    /// Get left entity that's related with thethe right `child_entity`.
    pub fn parent_of(&self, child_entity: &XvcEntity) -> Result<(&ChildEntity<U, T>, &T)> {
        match self.child_parents.get(child_entity) {
            None => Err(Error::NoParentEntityFound {
                entity: (*child_entity),
            }),
            Some(p_e) => {
                let (_, v) = self
                    .parents
                    .get_key_value(p_e)
                    .ok_or(Error::NoParentEntityFound {
                        entity: *child_entity,
                    })?;
                Ok((p_e, v))
            }
        }
    }

    /// Remove the child entity from child-parent store and children store
    pub fn remove_child(&mut self, child_entity: XvcEntity) -> Result<()> {
        self.child_parents.remove(child_entity);
        self.children.remove(child_entity);
        Ok(())
    }
}

impl<T, U> R1NStore<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Loads the stores from store root and directory named after type names
    pub fn load_r1nstore(store_root: &Path) -> Result<R1NStore<T, U>> {
        let parents = XvcStore::<T>::load_store(store_root)?;
        let children = XvcStore::<U>::load_store(store_root)?;
        let child_parents = XvcStore::<ChildEntity<U, T>>::load_store(store_root)?;

        Ok(R1NStore {
            parents,
            children,
            child_parents,
        })
    }

    /// Records the stores to store root and directories created from type names
    pub fn save_r1nstore(store: &Self, store_root: &Path) -> Result<()> {
        store.parents.save(store_root)?;
        store.children.save(store_root)?;
        store.child_parents.save(store_root)
    }
}
