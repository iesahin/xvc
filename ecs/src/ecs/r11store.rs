use std::path::Path;

use crate::{error::Result, XvcStore};
use crate::{Storable, XvcEntity};
use std::fmt::Debug;
/// R11Store is 1-1 RelationStore, where we store relationships with identical entities. It doesn't have any semantics except binding two components with a single entity.
///
#[derive(Debug, Clone)]
pub struct R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    pub left: XvcStore<T>,
    pub right: XvcStore<U>,
}

impl<T, U> R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Creates an empty R11Store
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
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = 100usize.into();
    /// rs.insert(&entity, "left component".into(), "right component".to_string());
    /// ```
    pub fn left_to_right(&self, entity: &XvcEntity) -> Option<(&XvcEntity, &U)> {
        self.right.get_key_value(entity)
    }

    /// returns the left element in L-R pair
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = 100.into();
    /// rs.insert(&entity, "left component".into(), "right component".into());
    /// ```
    pub fn right_to_left(&self, entity: &XvcEntity) -> Option<(&XvcEntity, &T)> {
        self.left.get_key_value(entity)
    }

    /// Returns L-R as a tuple
    /// ```
    /// # use xvc_ecs::{R11Store, XvcEntity};
    /// # let mut rs = R11Store::<String, String>::new();
    /// let entity: XvcEntity = 100.into();
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
            Some(vec_e) => vec_e.get(0).copied(),
        }
    }

    /// removes the components from both right and left
    pub fn remove(&mut self, entity: XvcEntity) {
        self.left.remove(entity);
        self.right.remove(entity);
    }

    pub fn append(&mut self, other: &R11Store<T, U>) -> Result<()> {
        self.left.append(&other.left)?;
        self.right.append(&other.right)
    }
}

impl<T, U> R11Store<T, U>
where
    T: Storable,
    U: Storable,
{
    pub fn load_r11store(store_root: &Path) -> Result<R11Store<T, U>> {
        let left = XvcStore::<T>::load_store(store_root)?;

        let right = XvcStore::<U>::load_store(store_root)?;

        Ok(R11Store { left, right })
    }

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
        let entity: XvcEntity = 100.into();
        rs.insert(&entity, "left component".into(), "right component".into());
        assert!(rs.left[&entity] == "left component");
        assert!(rs.right[&entity] == "right component");
        Ok(())
    }

    #[test]
    fn test_left_to_right() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = 100usize.into();
        rs.insert(
            &entity,
            "left component".into(),
            "right component".to_string(),
        );
        assert!(rs.left_to_right(&entity) == Some((&entity, &"right component".to_string())));
        assert!(rs.left_to_right(&(101usize.into())) == None);
        Ok(())
    }
    #[test]
    fn test_right_to_left() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = 100usize.into();
        rs.insert(&entity, "left component".into(), "right component".into());
        assert!(rs.right_to_left(&entity) == Some((&entity, &"left component".to_string())));
        assert!(rs.right_to_left(&101usize.into()) == None);
        Ok(())
    }

    #[test]
    fn test_tuple() -> Result<()> {
        let mut rs = R11Store::<String, String>::new();
        let entity: XvcEntity = 100usize.into();
        rs.insert(&entity, "left component".into(), "right component".into());
        let t = rs.tuple(&entity);
        assert!(t.0.as_deref() == Some(&"left component".to_string()));
        assert!(t.1.as_deref() == Some(&"right component".to_string()));
        Ok(())
    }
}
