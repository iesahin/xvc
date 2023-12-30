//! R11Store is 1-1 RelationStore.
//! We store relationships with identical entities.
//! It doesn't have any semantics except binding two components with a single entity.
use std::path::Path;

use crate::{error::Result, XvcStore};
use crate::{Storable, XvcEntity};
use std::fmt::Debug;

/// Associates two [XvcStore]s with two different type of components with a single [XvcEntity].
/// It's like using the same primary key in two database tables.
#[derive(Debug, Clone)]
pub struct R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    /// The first XvcStore to be associated
    pub left: XvcStore<T>,
    /// The second XvcStore to be associated
    pub right: XvcStore<U>,
}

impl<T, U> R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Creates an empty R11Store
    ///
    /// The following creates two new stores: `XvcStore<String>` and `XvcStore<i32>` that can be
    /// used in parallel with the same [`XvcEntity`] keys.
    ///
    /// ```
    /// use xvc_ecs::R11Store;
    /// let rs = R11Store::<String, i32>::new();
    /// ```

    pub fn new() -> Self {
        Self {
            left: XvcStore::<T>::new(),
            right: XvcStore::<U>::new(),
        }
    }

    /// inserts an element to both left and right
    ///
    /// Having a R11Store<String, String>, the following code inserts "left component" and "right
    /// component" with the same `XvcEntity(100)`.
    ///
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = 100.into();
    /// rs.insert(&entity, "left component".into(), "right component".into());
    /// ```
    pub fn insert(&mut self, entity: &XvcEntity, left_component: T, right_component: U) {
        self.left.insert(*entity, left_component);
        self.right.insert(*entity, right_component);
    }

    /// returns the right element in L-R pair
    ///
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = (100u64, 200u64).into();
    /// rs.insert(&entity, "left component".into(), "right component".to_string());
    /// ```

    pub fn left_to_right(&self, entity: &XvcEntity) -> Option<(&XvcEntity, &U)> {
        self.right.get_key_value(entity)
    }

    /// returns the left element in L-R pair
    ///
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = (100, 200).into();
    /// rs.insert(&entity, "left component".into(), "right component".into());
    /// ```
    pub fn right_to_left(&self, entity: &XvcEntity) -> Option<(&XvcEntity, &T)> {
        self.left.get_key_value(entity)
    }

    /// Returns L-R as a tuple
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = (100, 200).into();
    /// rs.insert(&entity, "left component".into(), "right component".into());
    /// let t = rs.tuple(&entity);
    /// ```

    pub fn tuple(&self, entity: &XvcEntity) -> (Option<&T>, Option<&U>) {
        (self.left.get(entity), self.right.get(entity))
    }

    /// Finds the entity from the left value
    pub fn entity_by_left(&self, left_element: &T) -> Option<XvcEntity> {
        match self.left.entities_for(left_element) {
            Some(entities) => {
                if entities.len() == 1 {
                    Some(entities[0])
                } else if entities.is_empty() {
                    None
                } else {
                    panic!("Multiple entities found for {left_element:?}");
                }
            }
            None => None,
        }
    }

    /// Finds the first entity from the right value
    pub fn entity_by_right(&self, right_element: &U) -> Option<XvcEntity> {
        match self.right.entities_for(right_element) {
            None => None,
            Some(vec_e) => vec_e.first().copied(),
        }
    }

    /// removes the components from both right and left
    pub fn remove(&mut self, entity: XvcEntity) {
        self.left.remove(entity);
        self.right.remove(entity);
    }

    /// Search the right value by left
    pub fn lookup_by_left(&self, left_element: &T) -> Option<&U> {
        match self.left.entity_by_value(left_element) {
            None => None,
            Some(xe) => self.right.get(&xe),
        }
    }

    /// Search the left value by right
    pub fn lookup_by_right(&self, right_element: &U) -> Option<&T> {
        match self.right.entity_by_value(right_element) {
            None => None,
            Some(xe) => self.left.get(&xe),
        }
    }
}

impl<T, U> R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Creates a 1-1 store by loading member stores with [XvcStore::load_store]
    pub fn load_r11store(store_root: &Path) -> Result<R11Store<T, U>> {
        let left = XvcStore::<T>::load_store(store_root)?;
        let right = XvcStore::<U>::load_store(store_root)?;

        Ok(R11Store { left, right })
    }

    /// Records a store by recording the member stores with [XvcStore::save].
    pub fn save_r11store(&self, store_root: &Path) -> Result<()> {
        self.left.save(store_root)?;
        self.right.save(store_root)
    }
}

impl<T, U> Default for R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::error::Result;
    #[test]
    fn test_new() -> Result<()> {
        let rs1 = R11Store::<String, i32> {
            left: XvcStore::<String>::new(),
            right: XvcStore::<i32>::new(),
        };

        let rs2 = R11Store::<String, i32>::new();

        assert!(rs1.right.len() == rs2.right.len());
        assert!(rs1.left.len() == rs2.left.len());

        Ok(())
    }

    #[test]
    fn test_insert() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = (100, 12830912380).into();
        rs.insert(&entity, "left component".into(), "right component".into());
        assert!(rs.left[&entity] == "left component");
        assert!(rs.right[&entity] == "right component");
        Ok(())
    }

    #[test]
    fn test_left_to_right() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = (100, 218021380921).into();
        rs.insert(
            &entity,
            "left component".into(),
            "right component".to_string(),
        );
        assert!(rs.left_to_right(&entity) == Some((&entity, &"right component".to_string())));
        assert!(rs.left_to_right(&(101, 921309218309).into()).is_none());
        Ok(())
    }
    #[test]
    fn test_right_to_left() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = (100, 128012389012).into();
        rs.insert(&entity, "left component".into(), "right component".into());
        assert!(rs.right_to_left(&entity) == Some((&entity, &"left component".to_string())));
        assert!(rs.right_to_left(&(101, 8120938120931).into()).is_none());
        Ok(())
    }

    #[test]
    fn test_tuple() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = (100, 123980123819203).into();
        rs.insert(&entity, "left component".into(), "right component".into());
        let t = rs.tuple(&entity);
        assert!(t.0 == Some(&"left component".to_string()));
        assert!(t.1 == Some(&"right component".to_string()));
        Ok(())
    }
}
