//! This module contains PathSync structure to synchronize operations on paths.
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};

use crate::{AbsolutePath, Result};

/// When multiple threads try to access the same path (especially in cache operations) we get weird
/// race conditions, This structure is to make those operations atomic and thread safe.
#[derive(Debug, Default)]
pub struct PathSync {
    locks: Arc<RwLock<HashMap<PathBuf, Arc<Mutex<()>>>>>,
}

impl PathSync {
    /// Create a new PathSync
    pub fn new() -> Self {
        Self {
            locks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Locks the path and runs the given closure with it.
    pub fn with_sync_path(
        &self,
        path: &Path,
        mut f: impl FnMut(&Path) -> Result<()>,
    ) -> Result<()> {
        // Do not lock the whole HashMap
        let entry = {
            let locks = self.locks.clone();
            let mut locks = locks.write()?;
            locks
                .entry(path.to_path_buf())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };
        let _guard = entry.lock()?;
        f(path)?;
        Ok(())
    }
    /// Locks the path and runs the given closure with it.
    pub fn with_sync_abs_path(
        &self,
        path: &AbsolutePath,
        mut f: impl FnMut(&AbsolutePath) -> Result<()>,
    ) -> Result<()> {
        {
            // Do not lock the whole HashMap
            let entry = {
                let locks = self.locks.clone();
                let mut locks = locks.write()?;
                locks
                    .entry(path.to_path_buf())
                    .or_insert_with(|| Arc::new(Mutex::new(())))
                    .clone()
            };
            let _guard = entry.lock()?;
            f(path)?;
        }

        Ok(())
    }
}

/// A thread safe singleton for PathSync
pub type PathSyncSingleton = Arc<Mutex<PathSync>>;
