use serde::{Deserialize, Serialize};
use xvc_ecs::persist;

use super::{XvcRemoteGuid, XvcRemotePath};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteInitEvent {
    pub guid: XvcRemoteGuid,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteListEvent {
    pub guid: XvcRemoteGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteSendEvent {
    pub guid: XvcRemoteGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteReceiveEvent {
    pub guid: XvcRemoteGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemoteDeleteEvent {
    pub guid: XvcRemoteGuid,
    pub paths: Vec<XvcRemotePath>,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcRemoteEvent {
    Init(XvcRemoteInitEvent),
    List(XvcRemoteListEvent),
    Send(XvcRemoteSendEvent),
    Receive(XvcRemoteReceiveEvent),
    Delete(XvcRemoteDeleteEvent),
}
persist!(XvcRemoteEvent, "remote-event");
