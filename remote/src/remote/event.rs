use serde::{Deserialize, Serialize};
use xvc_ecs::persist;

use super::{XvcRemotePath, XvcStorageGuid};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageInitEvent {
    pub guid: XvcStorageGuid,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageListEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageSendEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageReceiveEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcStorageDeleteEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcStorageEvent {
    Init(XvcStorageInitEvent),
    List(XvcStorageListEvent),
    Send(XvcStorageSendEvent),
    Receive(XvcStorageReceiveEvent),
    Delete(XvcStorageDeleteEvent),
}
persist!(XvcStorageEvent, "remote-event");
