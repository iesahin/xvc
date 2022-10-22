#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod bstore;
pub mod event;
pub mod hstore;
pub mod r11store;
pub mod r1nstore;
pub mod rmnstore;
pub mod storable;
pub mod vstore;
pub mod xvcstore;

use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use xvc_logging::watch;

use crate::error::{Error as XvcError, Result as XvcResult};

/// Describes an entity in Entity Component System-sense.
///
/// It doesn't have any semantics except being a unique number for a given entity.
/// Various types of information (components) can be attached to this entity.
/// XvcStore uses the entity as a key for the components.
///
/// It's possible to convert to `usize` back and forth.
/// Normally, you should use [XvcEntityGenerator] to create entities.
/// It ensures that the numbers are unique and saves the last number across sessions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct XvcEntity(usize);

impl fmt::Display for XvcEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for XvcEntity {
    fn from(e: usize) -> Self {
        Self(e)
    }
}

impl From<XvcEntity> for usize {
    fn from(e: XvcEntity) -> usize {
        e.0
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
    current: AtomicUsize,
}

static INIT: Once = Once::new();

/// Loads the generator from a directory.
///
/// It loads all files from that directory, and selects the highest one.
/// This function can only be used once in a process.
/// You cannot load a second instance of the entity generator, as it will defeat its thread-safe
/// uniqueness purpose.
pub fn load_generator(dir: &Path) -> XvcResult<XvcEntityGenerator> {
    let mut gen: XvcResult<XvcEntityGenerator> = Err(XvcError::CanInitializeOnlyOnce {
        object: "XvcEntityGenerator".to_string(),
    });
    INIT.call_once(|| gen = XvcEntityGenerator::load(dir));
    gen
}

/// Inits a generator for the first time.
///
/// Normally this only be used once an Xvc repository initializes.
/// The starting value for entities is 1.
pub fn init_generator() -> XvcResult<XvcEntityGenerator> {
    let mut gen: XvcResult<XvcEntityGenerator> = Err(XvcError::CanInitializeOnlyOnce {
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
    fn new(start: usize) -> XvcEntityGenerator {
        let current = AtomicUsize::new(0);
        current.fetch_add(start, Ordering::SeqCst);
        Self { current }
    }

    pub fn next_element(&self) -> XvcEntity {
        XvcEntity(self.current.fetch_add(1, Ordering::SeqCst))
    }

    fn load(dir: &Path) -> XvcResult<XvcEntityGenerator> {
        let path = most_recent_file(dir)?;
        match path {
            Some(path) => {
                let current_val = fs::read_to_string(path)?.parse::<usize>()?;
                Ok(Self::new(current_val))
            }
            None => Err(XvcError::CannotRestoreEntityCounter {
                path: dir.as_os_str().to_owned(),
            }),
        }
    }

    /// saves current_value to `path`
    pub fn save(&self, dir: &Path) -> XvcResult<()> {
        let u: usize = self.next_element().into();
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        let path = dir.join(timestamp());
        fs::write(path, format!("{}", u))?;
        Ok(())
    }
}

/// Returns a timestamp string to be used in file names.
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
pub fn sorted_files(dir: &Path) -> XvcResult<Vec<PathBuf>> {
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
        fs::create_dir_all(dir);
        Ok(vec![])
    }
}

/// This one returns the most recent timestamp named file.
/// It gets files with [sorted_files] and returns the last one if there is one.
/// If there are no files in a directory, this returns `Ok(None)`.
pub fn most_recent_file(dir: &Path) -> XvcResult<Option<PathBuf>> {
    watch!(dir);
    if !dir.exists() {
        return Ok(None);
    }

    let files = sorted_files(dir)?;

    if files.is_empty() {
        Ok(None)
    } else {
        watch!(files);
        Ok(files.get(files.len() - 1).cloned())
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
    use xvc_logging::{setup_logging, watch};

    #[test]
    fn test_init() -> XvcResult<()> {
        let gen = init_generator()?;
        assert_eq!(gen.current.load(Ordering::SeqCst), 1);
        assert_eq!(gen.next_element(), XvcEntity(1));
        assert_eq!(gen.next_element(), XvcEntity(2));
        let gen2 = init_generator();
        assert!(matches!(gen2, Err(XvcError::CanInitializeOnlyOnce { .. })));
        Ok(())
    }

    #[test]
    fn test_load() -> XvcResult<()> {
        setup_logging(Some(LevelFilter::Trace), None);
        let tempdir = TempDir::new("test-xvc-ecs")?;
        let gen_dir = tempdir.path().join("entity-gen");
        fs::create_dir_all(&gen_dir)?;
        let r: usize = rand::random();
        let gen_file_1 = gen_dir.join(timestamp());
        fs::write(&gen_file_1, format!("{}", r))?;
        sleep(Duration::from_millis(1));
        let gen_file_2 = gen_dir.join(timestamp());
        fs::write(&gen_file_2, format!("{}", r + 1000))?;
        sleep(Duration::from_millis(1));
        let gen_file_3 = gen_dir.join(timestamp());
        fs::write(&gen_file_3, format!("{}", r + 2000))?;
        let gen = XvcEntityGenerator::load(&gen_dir)?;
        assert_eq!(gen.current.load(Ordering::SeqCst), r + 2000);
        assert_eq!(gen.next_element(), XvcEntity(r + 2000));
        assert_eq!(gen.next_element(), XvcEntity(r + 2001));
        assert_eq!(gen.next_element(), XvcEntity(r + 2002));
        gen.save(&gen_dir)?;
        let new_val = fs::read_to_string(most_recent_file(&gen_dir)?.unwrap())?.parse::<usize>()?;
        assert_eq!(new_val, r + 2003);
        Ok(())
    }
}
