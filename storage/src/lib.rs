#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod error;
pub mod storage;

use std::path::PathBuf;
use std::str::FromStr;

pub use crate::error::{Error, Result};
use clap::{Parser, Subcommand};

use crossbeam_channel::Sender;
use derive_more::Display;
pub use storage::{
    XvcLocalStorage, XvcStorage, XvcStorageEvent, XvcStorageGuid, XvcStorageOperations,
};

use xvc_ecs;
use xvc_ecs::XvcStore;

use xvc_core::XvcRoot;
use xvc_logging::XvcOutputLine;

/// Storage (on the cloud) management commands
#[derive(Debug, Parser)]
#[clap(name = "storage", about = "")]
pub struct StorageCLI {
    #[clap(subcommand)]
    pub subcommand: StorageSubCommand,
}

/// Remote subcommands
#[derive(Debug, Clone, Parser)]
#[clap(about = "Manage storages containing tracked file content")]
pub enum StorageSubCommand {
    /// list all remotes
    #[clap()]
    List,
    /// Remove a remote
    #[clap()]
    Remove {
        /// Name of the remote to be deleted
        #[clap(long)]
        name: String,
    },

    #[clap(subcommand)]
    New(StorageNewSubCommand),
}

/// Add new remotes
#[derive(Debug, Clone, Subcommand)]
#[clap(about = "add new remotes")]
pub enum StorageNewSubCommand {
    /// add a new local remote
    #[clap()]
    Local {
        /// Directory to be set as a remote
        #[clap(long)]
        path: PathBuf,
        #[clap(long, short)]
        name: String,
    },

    /// add a new generic remote
    #[clap()]
    Generic {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// Command to initialize the remote. This command is run once after defining the remote.
        ///
        /// You can use {URL} and {DIR}  as shortcuts.
        #[clap(long = "init", short = 'i')]
        init_command: String,
        /// Command to list the files in remote
        ///
        /// You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.
        #[clap(long = "list", short = 'l')]
        list_command: String,
        /// Command to download a file from remote.
        ///
        /// You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.
        #[clap(long = "download", short = 'd')]
        download_command: String,
        /// Command to upload a file to remote.
        ///
        /// You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.
        #[clap(long = "upload", short = 'u')]
        upload_command: String,
        /// The delete command to remove a file from remote
        /// You can use {URL} and {DIR} placeholders and define values for these with --url and --dir options.
        #[clap(long = "delete", short = 'D')]
        delete_command: String,
        /// Number of maximum processes to run simultaneously
        #[clap(long = "processes", short = 'M', default_value_t = 1)]
        max_processes: usize,
        /// You can set a string to replace {URL} placeholder in commands
        #[clap(long)]
        url: Option<String>,
        /// You can set a string to replace {DIR} placeholder in commands
        #[clap(long)]
        storage_dir: Option<String>,
    },

    #[cfg(feature = "s3")]
    /// Add a new S3 remote
    #[clap()]
    S3 {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        remote_prefix: String,
        /// S3 bucket name
        #[clap(long)]
        bucket_name: String,
        /// AWS region
        #[clap(long)]
        region: String,
    },

    #[cfg(feature = "minio")]
    /// Add a new Minio remote
    #[clap()]
    Minio {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// Minio server url in the form https://myserver.example.com:9090
        #[clap(long)]
        endpoint: String,
        /// Bucket name
        #[clap(long)]
        bucket_name: String,
        /// Region of the server
        #[clap(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        remote_prefix: String,
    },

    #[cfg(feature = "digital-ocean")]
    /// Add a new Digital Ocean remote
    #[clap()]
    DigitalOcean {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[clap(long)]
        bucket_name: String,
        /// Region of the server
        #[clap(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        remote_prefix: String,
    },

    #[cfg(feature = "r2")]
    /// Add a new R2 remote
    #[clap()]
    R2 {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// R2 account ID
        #[clap(long)]
        account_id: String,
        /// Bucket name
        #[clap(long)]
        bucket_name: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        remote_prefix: String,
    },

    #[cfg(feature = "gcs")]
    /// Add a new Google Cloud Storage remote
    #[clap()]
    Gcs {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[clap(long)]
        bucket_name: String,
        /// Region of the server, e.g., europe-west3
        #[clap(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        remote_prefix: String,
    },

    #[cfg(feature = "wasabi")]
    /// Add a new Wasabi remote
    #[clap()]
    Wasabi {
        /// Name of the remote
        ///
        /// This must be unique among all remotes of the project
        #[clap(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[clap(long)]
        bucket_name: String,
        /// Endpoint for the server, complete with the region if there is
        ///
        /// e.g. for eu-central-1 region, use s3.eu-central-1.wasabisys.com as the endpoint.
        #[clap(long, default_value = "s3.wasabisys.com")]
        endpoint: String,
        /// You can set a directory in the bucket with this prefix
        #[clap(long, default_value = "")]
        storage_prefix: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum StorageIdentifier {
    Name(String),
    Uuid(uuid::Uuid),
}

impl FromStr for StorageIdentifier {
    fn from_str(s: &str) -> Result<Self> {
        match uuid::Uuid::parse_str(s) {
            Ok(uuid) => Ok(Self::Uuid(uuid)),
            Err(_) => Ok(Self::Name(s.to_string())),
        }
    }

    type Err = crate::Error;
}

pub fn cmd_storage(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    opts: StorageCLI,
) -> Result<()> {
    match opts.subcommand {
        StorageSubCommand::List => cmd_storage_list(input, output_snd, xvc_root),
        StorageSubCommand::Remove { name } => cmd_storage_remove(input, output_snd, xvc_root, name),
        StorageSubCommand::New(new) => cmd_remote_new(input, output_snd, xvc_root, new),
    }
}

fn cmd_remote_new(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    sc: StorageNewSubCommand,
) -> Result<()> {
    match sc {
        StorageNewSubCommand::Local { path, name } => {
            storage::local::cmd_storage_new_local(input, output_snd, xvc_root, path, name)
        }
        StorageNewSubCommand::Generic {
            name,
            init_command,
            list_command,
            download_command,
            upload_command,
            delete_command,
            max_processes,
            url,
            storage_dir: remote_dir,
        } => storage::generic::cmd_storage_new_generic(
            input,
            output_snd,
            xvc_root,
            name,
            url,
            remote_dir,
            max_processes,
            init_command,
            list_command,
            download_command,
            upload_command,
            delete_command,
        ),
        #[cfg(feature = "s3")]
        StorageNewSubCommand::S3 {
            name,
            remote_prefix,
            bucket_name,
            region,
        } => storage::s3::cmd_new_s3(
            input,
            output_snd,
            xvc_root,
            name,
            region,
            bucket_name,
            remote_prefix,
        ),
        #[cfg(feature = "minio")]
        StorageNewSubCommand::Minio {
            name,
            endpoint,
            bucket_name,
            remote_prefix,
            region,
        } => storage::minio::cmd_new_minio(
            input,
            output_snd,
            xvc_root,
            name,
            endpoint,
            bucket_name,
            region,
            remote_prefix,
        ),
        #[cfg(feature = "digital-ocean")]
        StorageNewSubCommand::DigitalOcean {
            name,
            bucket_name,
            region,
            remote_prefix,
        } => storage::digital_ocean::cmd_new_digital_ocean(
            input,
            output_snd,
            xvc_root,
            name,
            bucket_name,
            region,
            remote_prefix,
        ),
        #[cfg(feature = "r2")]
        StorageNewSubCommand::R2 {
            name,
            account_id,
            bucket_name,
            remote_prefix,
        } => storage::r2::cmd_new_r2(
            input,
            output_snd,
            xvc_root,
            name,
            account_id,
            bucket_name,
            remote_prefix,
        ),
        #[cfg(feature = "gcs")]
        StorageNewSubCommand::Gcs {
            name,
            bucket_name,
            region,
            remote_prefix,
        } => storage::gcs::cmd_new_gcs(
            input,
            output_snd,
            xvc_root,
            name,
            bucket_name,
            region,
            remote_prefix,
        ),
        #[cfg(feature = "wasabi")]
        StorageNewSubCommand::Wasabi {
            name,
            bucket_name,
            endpoint,
            storage_prefix,
        } => storage::wasabi::cmd_new_wasabi(
            input,
            output_snd,
            xvc_root,
            name,
            bucket_name,
            endpoint,
            storage_prefix,
        ),
    }
}

fn cmd_storage_remove(
    input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    name: String,
) -> Result<()> {
    todo!()
}

fn cmd_storage_list(
    _input: std::io::StdinLock,
    output_snd: Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
) -> Result<()> {
    let store: XvcStore<XvcStorage> = xvc_root.load_store()?;

    for (_, s) in store.iter() {
        output_snd.send(XvcOutputLine::Output(format!("{}\n", s)))?;
    }

    Ok(())
}
