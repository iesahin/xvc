//! ECS utilities for easy load and save in a repository.
use crate::error::Result;
use crate::types::xvcroot::XvcRootInner;
use xvc_ecs::R11Store;
use xvc_ecs::R1NStore;
use xvc_ecs::RMNStore;
use xvc_ecs::Storable;
use xvc_ecs::XvcStore;

impl XvcRootInner {
    /// Load XvcStore from the repository stores
    pub fn load_store<T>(&self) -> Result<XvcStore<T>>
    where
        T: Storable,
    {
        Ok(XvcStore::<T>::load_store(self.store_dir())?)
    }

    /// Utility function to save a [xvc_ecs::XvcStore] in a repository.
    pub fn save_store<T>(&self, store: &XvcStore<T>) -> Result<()>
    where
        T: Storable,
    {
        Ok(XvcStore::<T>::save_store(store, self.store_dir())?)
    }

    /// Loads a [xvc_ecs::XvcStore] and runs the given closure with it.
    pub fn with_store<T>(&self, mut f: impl FnMut(&XvcStore<T>) -> Result<()>) -> Result<()>
    where
        T: Storable,
    {
        let store = self.load_store::<T>()?;
        f(&store)
    }

    /// Loads a [xvc_ecs::XvcStore], runs the given closure with it and saves the store.
    /// Closure is used to modify the store elements.
    pub fn with_store_mut<T>(&self, mut f: impl FnMut(&mut XvcStore<T>) -> Result<()>) -> Result<()>
    where
        T: Storable,
    {
        let mut store = self.load_store::<T>()?;
        f(&mut store)?;
        self.save_store(&store)
    }

    /// Utility function to load a [xvc_ecs::R11Store] in a repository.
    pub fn load_r11store<T, U>(&self) -> Result<R11Store<T, U>>
    where
        T: Storable,
        U: Storable,
    {
        Ok(R11Store::<T, U>::load_r11store(self.store_dir())?)
    }

    /// Utility function to load a [xvc_ecs::R1NStore] in a repository.
    pub fn load_r1nstore<T, U>(&self) -> Result<R1NStore<T, U>>
    where
        T: Storable,
        U: Storable,
    {
        Ok(R1NStore::<T, U>::load_r1nstore(self.store_dir())?)
    }

    /// Utility function to load a [xvc_ecs::RMNStore] in a repository.
    pub fn load_rmnstore<T, U>(&self) -> Result<RMNStore<T, U>>
    where
        T: Storable,
        U: Storable,
    {
        Ok(RMNStore::<T, U>::load_rmnstore(self.store_dir())?)
    }

    /// Utility function to save a [xvc_ecs::R11Store] in a repository.
    pub fn save_r11store<T, U>(&self, store: &R11Store<T, U>) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        Ok(R11Store::<T, U>::save_r11store(store, self.store_dir())?)
    }

    /// Utility function to save a [xvc_ecs::R1NStore] in a repository.
    pub fn save_r1nstore<T, U>(&self, store: &R1NStore<T, U>) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        Ok(R1NStore::<T, U>::save_r1nstore(store, self.store_dir())?)
    }

    /// Utility function to save a [xvc_ecs::RMNStore] in a repository.
    pub fn save_rmnstore<T, U>(&self, store: &RMNStore<T, U>) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        Ok(RMNStore::<T, U>::save_rmnstore(store, self.store_dir())?)
    }

    /// Loads a [xvc_ecs::R11Store] and runs the given closure with it.
    pub fn with_r11store<T, U>(
        &self,
        mut f: impl FnMut(&R11Store<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let store = self.load_r11store::<T, U>()?;
        f(&store)
    }

    /// Loads a [xvc_ecs::R1NStore] and runs the given closure with it.
    pub fn with_r1nstore<T, U>(
        &self,
        mut f: impl FnMut(&R1NStore<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let store = self.load_r1nstore::<T, U>()?;
        f(&store)
    }

    /// Loads a [xvc_ecs::RMNStore] and runs the given closure with it.
    pub fn with_rmnstore<T, U>(
        &self,
        mut f: impl FnMut(&RMNStore<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let store = self.load_rmnstore::<T, U>()?;
        f(&store)
    }

    /// Loads a [xvc_ecs::R11Store], runs the given closure with it and saves the store.
    /// Closure is used to modify the store elements.
    pub fn with_r11store_mut<T, U>(
        &self,
        f: impl Fn(&mut R11Store<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let mut store = self.load_r11store::<T, U>()?;

        f(&mut store)?;

        self.save_r11store::<T, U>(&store)
    }
    /// Loads a [xvc_ecs::R1NStore], runs the given closure with it and saves the store.
    /// Closure is used to modify the store elements.
    pub fn with_r1nstore_mut<T, U>(
        &self,
        f: impl Fn(&mut R1NStore<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let mut store = self.load_r1nstore::<T, U>()?;
        f(&mut store)?;
        self.save_r1nstore::<T, U>(&store)
    }
    /// Loads a [xvc_ecs::RMNStore], runs the given closure with it and saves the store.
    /// Closure is used to modify the store elements.
    pub fn with_rmnstore_mut<T, U>(
        &self,
        f: impl Fn(&mut RMNStore<T, U>) -> Result<()>,
    ) -> Result<()>
    where
        T: Storable,
        U: Storable,
    {
        let mut store = self.load_rmnstore::<T, U>()?;
        f(&mut store)?;
        self.save_rmnstore::<T, U>(&store)
    }
}
