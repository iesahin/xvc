//! Xvc Entity Component System allows arbitrary [serializable][Storable] components associated
//! with [entities][XvcEntities] of integers.
//! It's used instead of _object-oriented_ architecture for flexible and maintainable features.
//!
//! In Xvc-ECS, each entity is a plain integer.
//! Components are arbitrary structs implementing [Storable], and [stores][XvcStore] are systems
//! that we use to represent associations between entity and components, and between entities.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod event;
pub mod hstore;
pub mod r11store;
pub mod r1nstore;
pub mod rmnstore;
pub mod storable;
pub mod vstore;
pub mod xvcstore;

use rand::{rngs, RngCore, SeedableRng};
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Once;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use xvc_logging::watch;

use crate::error::{Error as XvcError, Result};

/// Describes an entity in Entity Component System-sense.
///
/// It doesn't have any semantics except being unique for a given entity.
/// Various types of information (components) can be attached to this entity.
/// XvcStore uses the entity as a key for the components.
///
/// It's possible to convert to `(u64, u64)` or `u128` back and forth.
/// Normally, you should use [XvcEntityGenerator] to create entities.
/// It randomizes the first value to be unique and saves the last number across sessions.
/// This changed in 0.5. See https://github.com/iesahin/xvc/issues/198
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct XvcEntity(u64, u64);

impl fmt::Display for XvcEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(u64, u64)> for XvcEntity {
    fn from(e: (u64, u64)) -> Self {
        Self(e.0, e.1)
    }
}

impl From<u128> for XvcEntity {
    fn from(e: u128) -> Self {
        Self((e >> 64) as u64, e as u64)
    }
}

impl From<XvcEntity> for u128 {
    fn from(e: XvcEntity) -> u128 {
        ((e.0 as u128) << 64) | (e.1 as u128)
    }
}

impl From<XvcEntity> for (u64, u64) {
    fn from(e: XvcEntity) -> (u64, u64) {
        (e.0, e.1)
    }
}

/// Keeps track of the latest [XvcEntity] values, and creates new entities.
///
/// It's thread safe as it uses [AtomicUsize] for the highest entity value.
/// It can save itself to a file, loads from file.
/// Only one instance of this created for each app.
/// It runs load or init functions via [Once] and doesn't allow to load a second instance.

#[derive(Debug)]
pub struct XvcEntityGenerator {
    /// The counter is used to generate the first portion of new entities. It's incremented at every [`next_element`] call.
    counter: AtomicU64,
    /// The random value is used to generate the second portion of new entities. It's generated once at the
    /// initialization of this struct and never changed.
    random: u64,
    /// The counter is saved only if its value is changed.
    dirty: AtomicBool,
}

static INIT: Once = Once::new();

/// Loads the generator from a directory.
///
/// It loads all files from that directory, and selects the highest one.
/// This function can only be used once in a process.
/// You cannot load a second instance of the entity generator, as it will defeat its thread-safe
/// uniqueness purpose.
pub fn load_generator(dir: &Path) -> Result<XvcEntityGenerator> {
    let mut gen: Result<XvcEntityGenerator> = Err(XvcError::CanInitializeOnlyOnce {
        object: "XvcEntityGenerator".to_string(),
    });
    INIT.call_once(|| gen = XvcEntityGenerator::load(dir));
    gen
}

/// Inits a generator for the first time.
///
/// Normally this only be used once an Xvc repository initializes.
/// The starting value for entities is 1.
pub fn init_generator() -> Result<XvcEntityGenerator> {
    let mut gen: Result<XvcEntityGenerator> = Err(XvcError::CanInitializeOnlyOnce {
        object: "XvcEntityGenerator".to_string(),
    });

    INIT.call_once(|| gen = Ok(XvcEntityGenerator::new(1)));
    gen
}

impl Iterator for XvcEntityGenerator {
    type Item = XvcEntity;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_element())
    }
}

impl XvcEntityGenerator {
    fn new(start: u64) -> XvcEntityGenerator {
        let counter = AtomicU64::new(start);
        let mut rng = rngs::StdRng::from_entropy();
        let init_random = rng.next_u64();
        // When we create a new generator from scratch, we need to save it.
        // In the load function, we set this to false, as we don't want to save duplicate values.
        let dirty = AtomicBool::new(true);
        Self {
            dirty,
            counter,
            random: init_random,
        }
    }

    /// Returns the next element by atomically incresing the current value.
    pub fn next_element(&self) -> XvcEntity {
        self.dirty.store(true, Ordering::SeqCst);
        XvcEntity(self.counter.fetch_add(1, Ordering::SeqCst), self.random)
    }

    fn load(dir: &Path) -> Result<XvcEntityGenerator> {
        let path = most_recent_file(dir)?;
        match path {
            Some(path) => {
                let current_val = fs::read_to_string(path)?.parse::<u64>()?;
                // We don't use new here to set the dirty flag to false.
                let counter = AtomicU64::new(current_val);
                let mut rng = rngs::StdRng::from_entropy();
                let init_random = rng.next_u64();
                // When we load a new generator from file, we don't need to save it.
                // In the new function, we set this to true, as we need to save the first value.
                let dirty = AtomicBool::new(false);
                Ok(Self {
                    dirty,
                    counter,
                    random: init_random,
                })
            }
            None => Err(XvcError::CannotRestoreEntityCounter {
                path: dir.as_os_str().to_owned(),
            }),
        }
    }

    /// Saves the current XvcEntity counter to path.
    /// It saves only the first (e.0) part of the entity. The second part is
    /// generated randomly to randomize entities in different invocations of the app.
    pub fn save(&self, dir: &Path) -> Result<()> {
        if self.dirty.load(Ordering::SeqCst) {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
            let path = dir.join(timestamp());
            fs::write(path, format!("{}", self.counter.load(Ordering::SeqCst)))?;
            // We don't need to save again until changed.
            self.dirty.store(false, Ordering::SeqCst);
        }
        Ok(())
    }
}

/// Returns a timestamp string to be used in file names.
/// This is used to generate sortable unique file names in event logs.
pub fn timestamp() -> String {
    let now = SystemTime::now();
    let since = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!");
    format!("{}", since.as_micros())
}

/// Returns all files in a directory sorted by name.
/// This is used to sort timestamp named files. (See [timestamp]).
/// Store files are loaded in this order to replay the changes across branches.
/// TODO: Add link to book chapter.
pub fn sorted_files(dir: &Path) -> Result<Vec<PathBuf>> {
    if dir.exists() {
        let mut files: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|e| match e {
                Ok(e) => Some(e.path()),
                Err(_) => None,
            })
            .collect();

        files.sort_unstable();
        Ok(files)
    } else {
        fs::create_dir_all(dir)?;
        Ok(vec![])
    }
}

/// This one returns the most recent timestamp named file.
/// It gets files with [sorted_files] and returns the last one if there is one.
/// If there are no files in a directory, this returns `Ok(None)`.
pub fn most_recent_file(dir: &Path) -> Result<Option<PathBuf>> {
    watch!(dir);
    if !dir.exists() {
        return Ok(None);
    }

    let files = sorted_files(dir)?;

    if files.is_empty() {
        Ok(None)
    } else {
        watch!(files);
        Ok(files.last().cloned())
    }
}

#[macro_export]
/// Specifies the store name a type is stored.
/// Usually it's the string representation of a type.
/// These strings are used to generate store locations for types.
/// They must be unique across the project.
///
/// ## Example
/// ```rust,ignore
/// persist!(MyType, "my-type");
/// ```
///
/// makes `MyType` to implement [xvc_ecs::PersistentComponent].
/// This trait has a `type_description` function that returns the specified string.
/// The `type_description` string is then used to generate store identifiers like
/// `my-type-bstore.json` or `my-type-another-type-r11store.json`
macro_rules! persist {
    ( $t:ty, $desc:literal ) => {
        impl ::xvc_ecs::Storable for $t {
            fn type_description() -> String {
                $desc.to_string()
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use super::*;
    use log::LevelFilter;
    use rand;
    use tempdir::TempDir;
    use xvc_logging::setup_logging;

    #[test]
    fn test_init() -> Result<()> {
        let gen = init_generator()?;
        assert_eq!(gen.counter.load(Ordering::SeqCst), 1);
        assert_eq!(gen.next_element().0, 1);
        assert_eq!(gen.next_element().0, 2);
        let gen2 = init_generator();
        assert!(matches!(gen2, Err(XvcError::CanInitializeOnlyOnce { .. })));
        Ok(())
    }

    #[test]
    fn test_load() -> Result<()> {
        setup_logging(Some(LevelFilter::Trace), None);
        let tempdir = TempDir::new("test-xvc-ecs")?;
        let gen_dir = tempdir.path().join("entity-gen");
        fs::create_dir_all(&gen_dir)?;
        let r: u64 = rand::random();
        let gen_file_1 = gen_dir.join(timestamp());
        fs::write(gen_file_1, format!("{}", r))?;
        sleep(Duration::from_millis(1));
        let gen_file_2 = gen_dir.join(timestamp());
        fs::write(gen_file_2, format!("{}", r + 1000))?;
        sleep(Duration::from_millis(1));
        let gen_file_3 = gen_dir.join(timestamp());
        fs::write(gen_file_3, format!("{}", r + 2000))?;
        let gen = XvcEntityGenerator::load(&gen_dir)?;
        assert_eq!(gen.counter.load(Ordering::SeqCst), r + 2000);
        assert_eq!(gen.next_element().0, (r + 2000));
        assert_eq!(gen.next_element().0, (r + 2001));
        assert_eq!(gen.next_element().0, (r + 2002));
        gen.save(&gen_dir)?;
        let new_val = fs::read_to_string(most_recent_file(&gen_dir)?.unwrap())?.parse::<u64>()?;
        assert_eq!(new_val, r + 2003);
        Ok(())
    }

    /// Multiple saves without changed counter should not create new files.
    /// See: https://github.com/iesahin/xvc/issues/185
    #[test]
    fn test_multi_save() -> Result<()> {
        setup_logging(Some(LevelFilter::Trace), None);
        let tempdir = TempDir::new("test-xvc-ecs")?;
        let gen_dir = tempdir.path().join("entity-gen");
        fs::create_dir_all(&gen_dir)?;
        // We use new here to circumvent the singleton check.
        let gen = XvcEntityGenerator::new(10);
        gen.save(&gen_dir)?;
        // It must save the counter at first
        assert!(sorted_files(&gen_dir)?.len() == 1);
        // It must not save the counter if it's not changed
        gen.save(&gen_dir)?;
        assert!(sorted_files(&gen_dir)?.len() == 1);
        // It must save the counter if it's changed
        let _e = gen.next_element();
        gen.save(&gen_dir)?;
        assert!(sorted_files(&gen_dir)?.len() == 2);

        let gen2 = XvcEntityGenerator::load(&gen_dir)?;
        // Don't save if it's not changed after load
        gen.save(&gen_dir)?;
        assert!(sorted_files(&gen_dir)?.len() == 2);
        // Save if it's changed after load
        let _e = gen2.next_element();
        gen2.save(&gen_dir)?;
        assert!(sorted_files(&gen_dir)?.len() == 3);
        // Don't save if it's not changed after save
        gen2.save(&gen_dir)?;
        gen2.save(&gen_dir)?;
        gen2.save(&gen_dir)?;
        gen2.save(&gen_dir)?;
        assert!(sorted_files(&gen_dir)?.len() == 3);

        Ok(())
    }

    #[test]
    fn test_from_to() -> Result<()> {
        let e1 = XvcEntity(1, 2);
        let u1: u128 = e1.into();
        let e2 = XvcEntity::from(u1);
        assert_eq!(e1, e2);
        Ok(())
    }
}
