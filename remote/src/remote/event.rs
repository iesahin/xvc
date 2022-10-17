use serde::{Deserialize, Serialize};
use xvc_ecs::persist;

use super::{XvcRemotePath, XvcStorageGuid};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteInitEvent {
    pub guid: XvcStorageGuid,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteListEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteSendEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteReceiveEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteDeleteEvent {
    pub guid: XvcStorageGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcStorageEvent {
    Init(XvcRemoteInitEvent),
    List(XvcRemoteListEvent),
    Send(XvcRemoteSendEvent),
    Receive(XvcRemoteReceiveEvent),
    Delete(XvcRemoteDeleteEvent),
}
persist!(XvcStorageEvent, "remote-event");
