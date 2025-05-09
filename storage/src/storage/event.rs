//! Storage events that we track when we created the storage, what we sent, received and deleted.
use serde::{Deserialize, Serialize};
use xvc_core::persist;

use super::{XvcStorageGuid, XvcStoragePath};

/// The init event of a storage when the directory for is created and .xvc-guid file is written.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageInitEvent {
    /// The GUID written to the .xvc-guid file
    pub guid: XvcStorageGuid,
}

/// The list event when the storage contents are listed fully.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageListEvent {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// Elements in the storage.
    pub paths: Vec<XvcStoragePath>,
}

/// The send event when files are uploaded to the storage.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageSendEvent {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// Elements sent to the storage.
    pub paths: Vec<XvcStoragePath>,
}

/// The receive event when files are downloaded from the storage.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageReceiveEvent {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// Elements received from the storage.
    pub paths: Vec<XvcStoragePath>,
}

/// The delete event when files are deleted from the storage.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageDeleteEvent {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// Elements deleted from the storage.
    pub paths: Vec<XvcStoragePath>,
}
/// The share event when a file is shared via signed URL
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageExpiringShareEvent {
    /// The GUID of the storage
    pub guid: XvcStorageGuid,
    /// Elements shared
    pub path: XvcStoragePath,
    /// The signed URL
    pub signed_url: String,
    /// The expiration date of the signed URL
    pub expiration_seconds: u32,
}

/// Collected storage events.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcStorageEvent {
    /// The init event of a storage when the directory for is created and .xvc-guid file is written.
    Init(XvcStorageInitEvent),
    /// The list event when the storage contents are listed fully.
    List(XvcStorageListEvent),
    /// The send event when files are uploaded to the storage.
    Send(XvcStorageSendEvent),
    /// The receive event when files are downloaded from the storage.
    Receive(XvcStorageReceiveEvent),
    /// The delete event when files are deleted from the storage.
    Delete(XvcStorageDeleteEvent),
    /// The share event when a file is shared via signed URL
    Share(XvcStorageExpiringShareEvent),
}
persist!(XvcStorageEvent, "storage-event");
