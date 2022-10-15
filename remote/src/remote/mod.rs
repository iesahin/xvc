#[cfg(feature = "digital-ocean")]
pub mod digital_ocean;
pub mod event;
#[cfg(feature = "gcs")]
pub mod gcs;
pub mod generic;
pub mod local;
#[cfg(feature = "minio")]
pub mod minio;
#[cfg(feature = "r2")]
pub mod r2;
#[cfg(feature = "s3")]
pub mod s3;
#[cfg(feature = "wasabi")]
pub mod wasabi;

use std::{
    fmt::Display,
    fs::{self, create_dir_all, read_dir, write},
    path::{Path, PathBuf},
    str::FromStr,
};

use derive_more::{Display, FromStr};
pub use event::{
    XvcRemoteDeleteEvent, XvcRemoteEvent, XvcRemoteInitEvent, XvcRemoteListEvent,
    XvcRemoteReceiveEvent, XvcRemoteSendEvent,
};

pub use local::XvcLocalRemote;

use anyhow::anyhow;
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xvc_logging::XvcOutputLine;

use crate::{Error, RemoteIdentifier, Result};
use log::{debug, trace};
use relative_path::{RelativePath, RelativePathBuf};
use subprocess::Exec;
use xvc_core::{XvcCachePath, XvcFileType, XvcMetadata, XvcPath, XvcRoot};
use xvc_ecs::{persist, Storable, XvcEntity, XvcStore};

use self::generic::XvcGenericRemote;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum XvcRemote {
    Local(XvcLocalRemote),
    Generic(XvcGenericRemote),
    // Ssh(XvcSshRemote),
    #[cfg(feature = "s3")]
    S3(s3::XvcS3Remote),
    #[cfg(feature = "r2")]
    R2(r2::XvcR2Remote),
    #[cfg(feature = "gcs")]
    Gcs(gcs::XvcGcsRemote),
    #[cfg(feature = "minio")]
    Minio(minio::XvcMinioRemote),
    #[cfg(feature = "wasabi")]
    Wasabi(wasabi::XvcWasabiRemote),
    #[cfg(feature = "digital-ocean")]
    DigitalOcean(digital_ocean::XvcDigitalOceanRemote),
}
persist!(XvcRemote, "remote");

impl Display for XvcRemote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XvcRemote::Local(lr) => {
                write!(
                    f,
                    "Local:   {}\t{}\t{}",
                    lr.name,
                    lr.guid,
                    lr.path.to_string_lossy()
                )
            }
            XvcRemote::Generic(gr) => write!(
                f,
                "Generic: {}\t{}\t{}{}",
                gr.name,
                gr.guid,
                gr.url.as_ref().unwrap_or(&String::new()),
                gr.remote_dir.as_ref().unwrap_or(&String::new())
            ),

            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => write!(
                f,
                "S3:      {}\t{}\t{}.{}/{}",
                s3r.name, s3r.guid, s3r.region, s3r.bucket_name, s3r.remote_prefix
            ),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => write!(
                f,
                "Minio:   {}\t{}\t{}.{}/{}",
                mr.name, mr.guid, mr.endpoint, mr.bucket_name, mr.remote_prefix
            ),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r2r) => write!(
                f,
                "R2:      {}\t{}\t{} {}/{}",
                r2r.name, r2r.guid, r2r.account_id, r2r.bucket_name, r2r.remote_prefix
            ),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(gcsr) => write!(
                f,
                "GCS:     {}\t{}\t{}.{}/{}",
                gcsr.name, gcsr.guid, gcsr.region, gcsr.bucket_name, gcsr.remote_prefix
            ),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(wr) => write!(
                f,
                "Wasabi:  {}\t{}\t{}.{}/{}",
                wr.name, wr.guid, wr.region, wr.bucket_name, wr.remote_prefix
            ),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(dor) => write!(
                f,
                "DO:      {}\t{}\t{}.{}/{}",
                dor.name, dor.guid, dor.region, dor.bucket_name, dor.remote_prefix
            ),
        }
    }
}

pub trait XvcRemoteOperations {
    fn init(&self, output: Sender<XvcOutputLine>, xvc_root: &XvcRoot)
        -> Result<XvcRemoteInitEvent>;
    fn list(&self, output: Sender<XvcOutputLine>, xvc_root: &XvcRoot)
        -> Result<XvcRemoteListEvent>;
    fn send(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcRemoteSendEvent>;
    fn receive(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcRemoteReceiveEvent>;
    fn delete(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcRemoteDeleteEvent>;
}

impl XvcRemoteOperations for XvcRemote {
    fn init(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<XvcRemoteInitEvent> {
        match self {
            XvcRemote::Local(lr) => lr.init(output, xvc_root),
            XvcRemote::Generic(gr) => gr.init(output, xvc_root),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.init(output, xvc_root),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.init(output, xvc_root),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.init(output, xvc_root),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.init(output, xvc_root),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.init(output, xvc_root),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.init(output, xvc_root),
        }
    }

    fn list(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
    ) -> Result<XvcRemoteListEvent> {
        match self {
            XvcRemote::Local(lr) => lr.list(output, xvc_root),
            XvcRemote::Generic(gr) => gr.list(output, xvc_root),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.list(output, xvc_root),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.list(output, xvc_root),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.list(output, xvc_root),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.list(output, xvc_root),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.list(output, xvc_root),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.list(output, xvc_root),
        }
    }

    fn send(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcRemoteSendEvent> {
        match self {
            XvcRemote::Local(lr) => lr.send(output, xvc_root, paths, force),
            XvcRemote::Generic(gr) => gr.send(output, xvc_root, paths, force),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.send(output, xvc_root, paths, force),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.send(output, xvc_root, paths, force),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.send(output, xvc_root, paths, force),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.send(output, xvc_root, paths, force),
        }
    }

    fn receive(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
        force: bool,
    ) -> Result<XvcRemoteReceiveEvent> {
        match self {
            XvcRemote::Local(lr) => lr.receive(output, xvc_root, paths, force),
            XvcRemote::Generic(gr) => gr.receive(output, xvc_root, paths, force),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.receive(output, xvc_root, paths, force),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.receive(output, xvc_root, paths, force),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.receive(output, xvc_root, paths, force),
        }
    }

    fn delete(
        &self,
        output: Sender<XvcOutputLine>,
        xvc_root: &XvcRoot,
        paths: &[XvcCachePath],
    ) -> Result<XvcRemoteDeleteEvent> {
        match self {
            XvcRemote::Local(lr) => lr.delete(output, xvc_root, paths),
            XvcRemote::Generic(gr) => gr.delete(output, xvc_root, paths),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.delete(output, xvc_root, paths),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.delete(output, xvc_root, paths),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.delete(output, xvc_root, paths),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.delete(output, xvc_root, paths),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct XvcRemotePath(RelativePathBuf);
persist!(XvcRemotePath, "remote-path");

impl From<String> for XvcRemotePath {
    fn from(p: String) -> Self {
        Self(RelativePathBuf::from(p))
    }
}
impl FromStr for XvcRemotePath {
    fn from_str(p: &str) -> Result<Self> {
        Ok(Self(RelativePathBuf::from(p.to_string())))
    }

    type Err = crate::Error;
}

impl AsRef<RelativePath> for XvcRemotePath {
    fn as_ref(&self) -> &RelativePath {
        self.0.as_ref()
    }
}

impl XvcRemotePath {
    /// The remote path of a cache path is like {guid}/{cache-path}
    /// ⚠️  The separator between {guid} and {cache-path} is always /
    pub fn new(xvc_root: &XvcRoot, local: &XvcCachePath) -> Self {
        let guid = xvc_root.config().guid().unwrap();
        let guid_path = RelativePathBuf::from(guid);
        let rel_path = guid_path.join(local);
        Self(rel_path)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct XvcRemoteGuid(Uuid);

impl XvcRemoteGuid {
    pub fn new() -> Self {
        let guid = uuid::Uuid::new_v4();
        Self(guid)
    }
}

impl FromStr for XvcRemoteGuid {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for XvcRemoteGuid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

pub const XVC_REMOTE_GUID_FILENAME: &str = ".xvc-guid";

pub fn get_remote_from_store(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    identifier: &RemoteIdentifier,
) -> Result<XvcRemote> {
    let store: XvcStore<XvcRemote> = xvc_root.load_store()?;
    let remote_store = store.filter(|_, r| match identifier {
        RemoteIdentifier::Name(ref n) => match r {
            XvcRemote::Local(r) => r.name == *n,
            XvcRemote::Generic(r) => r.name == *n,
            #[cfg(feature = "s3")]
            XvcRemote::S3(r) => r.name == *n,
            #[cfg(feature = "minio")]
            XvcRemote::Minio(r) => r.name == *n,
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.name == *n,
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.name == *n,
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.name == *n,
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.name == *n,
        },
        RemoteIdentifier::Uuid(ref id) => match r {
            XvcRemote::Local(lr) => lr.guid == (*id).into(),
            XvcRemote::Generic(gr) => gr.guid == (*id).into(),
            #[cfg(feature = "s3")]
            XvcRemote::S3(s3r) => s3r.guid == (*id).into(),
            #[cfg(feature = "minio")]
            XvcRemote::Minio(mr) => mr.guid == (*id).into(),
            #[cfg(feature = "r2")]
            XvcRemote::R2(r) => r.guid == (*id).into(),
            #[cfg(feature = "gcs")]
            XvcRemote::Gcs(r) => r.guid == (*id).into(),
            #[cfg(feature = "wasabi")]
            XvcRemote::Wasabi(r) => r.guid == (*id).into(),
            #[cfg(feature = "digital-ocean")]
            XvcRemote::DigitalOcean(r) => r.guid == (*id).into(),
        },
    });

    if remote_store.is_empty() {
        output_snd
            .send(XvcOutputLine::Panic(format!(
                "Cannot find remote {}",
                identifier
            )))
            .unwrap();
    } else if remote_store.len() > 1 {
        output_snd
            .send(XvcOutputLine::Panic(format!(
                "Ambigious remote identifier: {}",
                identifier
            )))
            .unwrap();
    }

    let (_, remote) =
        remote_store
            .first()
            .ok_or_else(|| Error::CannotFindRemoteWithIdentifier {
                identifier: identifier.clone(),
            })?;
    Ok(remote.clone())
}
