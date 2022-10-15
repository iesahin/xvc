//! Key-value store for caching calculation results
use crate::types::xvcdigest::XvcDigest;
use crate::types::xvcmetadata::XvcMetadata;
use crate::types::xvcpath::XvcPath;
use crate::types::xvcroot::XvcRoot;
use crate::util::serde::{from_msgpack, to_msgpack};

use crate::error::{Error as XvcError, Result as XvcResult};
use sled;
use std::path::{Path, PathBuf};

/// The key-value store is saved under .xvc directory under the name of this constant.
pub const KV_STORE: &str = "kv";
const CONTENT_DIGEST_TREE: &str = "content-digest";

#[derive(Debug, Clone)]
/// A [sled::Db] based key-value store.
/// This is currently not used anywhere.
/// We can employ in certain scenarios after profiling.
pub struct XvcKeyValueStore {
    db: sled::Db,
}

impl XvcKeyValueStore {
    /// Load/create a new key-value store with the given `path`.
    fn new(path: &Path) -> XvcResult<XvcKeyValueStore> {
        let db = sled::open(path)?;

        Ok(XvcKeyValueStore { db })
    }

    /// Load/create a global key-value store for the repository.
    pub fn from_root(xvc_root: &XvcRoot) -> XvcResult<XvcKeyValueStore> {
        Self::new(&xvc_root.xvc_dir().join(&PathBuf::from(KV_STORE)))
    }

    /// Stores a `digest`, associated with `xvc_path` and `xvc_metadata` to prevent recalculation.
    /// It can be retrieved by [Self::get_content_digest].
    pub fn insert_content_digest(
        &self,
        xvc_path: &XvcPath,
        xvc_metadata: &XvcMetadata,
        digest: &XvcDigest,
    ) -> XvcResult<()> {
        let mut key = to_msgpack(xvc_path)?;
        key.extend(to_msgpack(xvc_metadata)?);
        let value = to_msgpack(digest)?;
        let tree = self.db.open_tree(CONTENT_DIGEST_TREE.as_bytes())?;
        tree.insert(key, value)?;
        Ok(())
    }

    /// Gets the content digest previously stored with [Self::insert_content_digest].
    pub fn get_content_digest(
        &self,
        xvc_path: &XvcPath,
        xvc_metadata: &XvcMetadata,
    ) -> XvcResult<Option<XvcDigest>> {
        let mut key = to_msgpack(xvc_path)?;
        key.extend(to_msgpack(xvc_metadata)?);
        let tree = self.db.open_tree(CONTENT_DIGEST_TREE.as_bytes())?;

        match tree.get(key) {
            Ok(None) => Ok(None),
            Ok(Some(ivec)) => Ok(Some(from_msgpack(&ivec)?)),
            Err(source) => Err(XvcError::SledError { source }),
        }
    }
}
