//! Xvc storage management commands.
//!
//! Contains several modules to implement connection, upload and download of files to a storage.
//! Most of the functionality is behind feature flags that are on by default. If you want to customize the functionality
//! of this crate, you can disable the features you don't need.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod error;
pub mod storage;

use std::path::PathBuf;
use std::str::FromStr;

pub use crate::error::{Error, Result};
use clap::{Parser, Subcommand};

use derive_more::Display;
pub use storage::{
    XvcLocalStorage, XvcStorage, XvcStorageEvent, XvcStorageGuid, XvcStorageOperations,
};

use xvc_ecs::XvcStore;

use xvc_core::XvcRoot;
use xvc_logging::{output, XvcOutputSender};

/// Storage (on the cloud) management commands
#[derive(Debug, Parser, Clone)]
#[command(name = "storage", about = "")]
pub struct StorageCLI {
    /// Subcommand for storage management
    #[command(subcommand)]
    pub subcommand: StorageSubCommand,
}

/// storage subcommands
#[derive(Debug, Clone, Parser)]
#[command(about = "Manage storages containing tracked file content")]
pub enum StorageSubCommand {
    /// List all configured storages
    #[command()]
    List,
    /// Remove a storage configuration.
    ///
    /// This doesn't delete any files in the storage.
    #[command()]
    Remove {
        /// Name of the storage to be deleted
        #[arg(long)]
        name: String,
    },

    /// Configure a new storage
    #[command(subcommand)]
    New(StorageNewSubCommand),
}

/// Add a new storage
#[derive(Debug, Clone, Subcommand)]
#[command()]
pub enum StorageNewSubCommand {
    /// Add a new local storage
    ///
    /// A local storage is a directory accessible from the local file system.
    /// Xvc will use common file operations for this directory without accessing the network.
    #[command()]
    Local {
        /// Directory (outside the repository) to be set as a storage
        #[arg(long)]
        path: PathBuf,
        /// Name of the storage.
        ///
        /// Recommended to keep this name unique to refer easily.
        #[arg(long, short)]
        name: String,
    },

    /// Add a new generic storage.
    ///
    /// ⚠️ Please note that this is an advanced method to configure storages.
    /// You may damage your repository and local and storage files with incorrect configurations.
    ///
    /// Please see https://docs.xvc.dev/ref/xvc-storage-new-generic.html for examples and make
    /// necessary backups.
    #[command()]
    Generic {
        /// Name of the storage.
        ///
        /// Recommended to keep this name unique to refer easily.
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Command to initialize the storage.
        /// This command is run once after defining the storage.
        ///
        /// You can use {URL} and {STORAGE_DIR}  as shortcuts.
        #[arg(long = "init", short = 'i')]
        init_command: String,
        /// Command to list the files in storage
        ///
        /// You can use {URL} and {STORAGE_DIR} placeholders and define values for these with --url and --storage_dir options.
        #[arg(long = "list", short = 'l')]
        list_command: String,
        /// Command to download a file from storage.
        ///
        /// You can use {URL} and {STORAGE_DIR} placeholders and define values for these with --url and --storage_dir options.
        #[arg(long = "download", short = 'd')]
        download_command: String,
        /// Command to upload a file to storage.
        ///
        /// You can use {URL} and {STORAGE_DIR} placeholders and define values for these with --url and --storage_dir options.
        #[arg(long = "upload", short = 'u')]
        upload_command: String,
        /// The delete command to remove a file from storage
        /// You can use {URL} and {STORAGE_DIR} placeholders and define values for these with --url and --storage_dir options.
        #[arg(long = "delete", short = 'D')]
        delete_command: String,
        /// Number of maximum processes to run simultaneously
        #[arg(long = "processes", short = 'M', default_value_t = 1)]
        max_processes: usize,
        /// You can set a string to replace {URL} placeholder in commands
        #[arg(long)]
        url: Option<String>,
        /// You can set a string to replace {STORAGE_DIR} placeholder in commands
        #[arg(long)]
        storage_dir: Option<String>,
    },

    /// Add a new rsync storages
    ///
    /// Uses rsync in separate processes to communicate.
    /// This can be used when you already have an SSH/Rsync connection.
    /// It doesn't prompt for any passwords. The connection must be set up with ssh keys beforehand.
    #[command()]
    Rsync {
        /// Name of the storage.
        ///
        /// Recommended to keep this name unique to refer easily.
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Hostname for the connection in the form host.example.com  (without @, : or protocol)
        #[arg(long)]
        host: String,
        /// Port number for the connection in the form 22.
        /// Doesn't add port number to connection string if not given.
        #[arg(long)]
        port: Option<usize>,
        /// User name for the connection, the part before @ in user@example.com (without @,
        /// hostname).
        /// User name isn't included in connection strings if not given.
        #[arg(long)]
        user: Option<String>,
        /// storage directory in the host to store the files.
        #[arg(long)]
        storage_dir: String,
    },

    #[cfg(feature = "s3")]
    /// Add a new S3 storage
    ///
    /// Reads credentials from `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    S3 {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
        /// S3 bucket name
        #[arg(long)]
        bucket_name: String,
        /// AWS region
        #[arg(long)]
        region: String,
    },

    #[cfg(feature = "minio")]
    /// Add a new Minio storage
    ///
    /// Reads credentials from `MINIO_ACCESS_KEY` and `MINIO_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    Minio {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Minio server url in the form https://myserver.example.com:9090
        #[arg(long)]
        endpoint: String,
        /// Bucket name
        #[arg(long)]
        bucket_name: String,
        /// Region of the server
        #[arg(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
    },

    #[cfg(feature = "digital-ocean")]
    /// Add a new Digital Ocean storage
    ///
    /// Reads credentials from `DIGITAL_OCEAN_ACCESS_KEY_ID` and `DIGITAL_OCEAN_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    DigitalOcean {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[arg(long)]
        bucket_name: String,
        /// Region of the server
        #[arg(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
    },

    #[cfg(feature = "r2")]
    /// Add a new R2 storage
    ///
    /// Reads credentials from `R2_ACCESS_KEY_ID` and `R2_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    R2 {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// R2 account ID
        #[arg(long)]
        account_id: String,
        /// Bucket name
        #[arg(long)]
        bucket_name: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
    },

    #[cfg(feature = "gcs")]
    /// Add a new Google Cloud Storage storage
    ///
    /// Reads credentials from `GCS_ACCESS_KEY_ID` and `GCS_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    Gcs {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[arg(long)]
        bucket_name: String,
        /// Region of the server, e.g., europe-west3
        #[arg(long)]
        region: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
    },

    #[cfg(feature = "wasabi")]
    /// Add a new Wasabi storage
    ///
    /// Reads credentials from `WASABI_ACCESS_KEY_ID` and `WASABI_SECRET_ACCESS_KEY` environment variables.
    /// Alternatively you can use `XVC_STORAGE_ACCESS_KEY_ID_<storage_name>` and
    /// `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` environment variables if you have multiple storages of this type.
    #[command()]
    Wasabi {
        /// Name of the storage
        ///
        /// This must be unique among all storages of the project
        #[arg(long = "name", short = 'n')]
        name: String,
        /// Bucket name
        #[arg(long)]
        bucket_name: String,
        /// Endpoint for the server, complete with the region if there is
        ///
        /// e.g. for eu-central-1 region, use s3.eu-central-1.wasabisys.com as the endpoint.
        #[arg(long, default_value = "s3.wasabisys.com")]
        endpoint: String,
        /// You can set a directory in the bucket with this prefix
        #[arg(long, default_value = "")]
        storage_prefix: String,
    },
}

/// Specifies a storage by either a name or a GUID.
///
/// Name is specified with `--name` option of most of the storage types.
/// Guid is generated or loaded in [XvcStorageOperations::init] operations and
/// kept in Storage structs.
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum StorageIdentifier {
    /// Name of the storage
    Name(String),
    /// GUID of the storage
    Uuid(uuid::Uuid),
}

impl FromStr for StorageIdentifier {
    /// This tries to parse `s` as a [Uuid]. If it can't it
    /// considers it a name.
    ///
    /// The only way this fails is when `s` cannot be converted to string.
    /// That's very unlikely.
    fn from_str(s: &str) -> Result<Self> {
        match uuid::Uuid::parse_str(s) {
            Ok(uuid) => Ok(Self::Uuid(uuid)),
            Err(_) => Ok(Self::Name(s.to_string())),
        }
    }

    type Err = crate::Error;
}

/// Entry point for `xvc storage` group of commands.
///
/// It matches the subcommand in [StorageCLI::subcommand] and runs the
/// appropriate function.
///
/// Other arguments are passed to subcommands.
pub fn cmd_storage(
    input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: StorageCLI,
) -> Result<()> {
    match opts.subcommand {
        StorageSubCommand::List => cmd_storage_list(input, output_snd, xvc_root),
        StorageSubCommand::Remove { name } => cmd_storage_remove(input, output_snd, xvc_root, name),
        StorageSubCommand::New(new) => cmd_storage_new(input, output_snd, xvc_root, new),
    }
}

/// Configure a new storage.
///
/// The available storages and their configuration is dependent to compilation.
/// In minimum, it includes [local][cmd_storage_new_local] and
/// [generic][cmd_storage_new_generic].
///
/// This function matches [StorageNewSubCommand] and calls the appropriate
/// function from child modules. Most of the available options are behind
/// feature flags, that also guard the modules.
fn cmd_storage_new(
    input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
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
            storage_dir,
        } => storage::generic::cmd_storage_new_generic(
            input,
            output_snd,
            xvc_root,
            name,
            url,
            storage_dir,
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
            storage_prefix,
            bucket_name,
            region,
        } => storage::s3::cmd_new_s3(
            output_snd,
            xvc_root,
            name,
            region,
            bucket_name,
            storage_prefix,
        ),
        #[cfg(feature = "minio")]
        StorageNewSubCommand::Minio {
            name,
            endpoint,
            bucket_name,
            storage_prefix,
            region,
        } => storage::minio::cmd_new_minio(
            input,
            output_snd,
            xvc_root,
            name,
            endpoint,
            bucket_name,
            region,
            storage_prefix,
        ),
        #[cfg(feature = "digital-ocean")]
        StorageNewSubCommand::DigitalOcean {
            name,
            bucket_name,
            region,
            storage_prefix,
        } => storage::digital_ocean::cmd_new_digital_ocean(
            input,
            output_snd,
            xvc_root,
            name,
            bucket_name,
            region,
            storage_prefix,
        ),
        #[cfg(feature = "r2")]
        StorageNewSubCommand::R2 {
            name,
            account_id,
            bucket_name,
            storage_prefix,
        } => storage::r2::cmd_new_r2(
            input,
            output_snd,
            xvc_root,
            name,
            account_id,
            bucket_name,
            storage_prefix,
        ),
        #[cfg(feature = "gcs")]
        StorageNewSubCommand::Gcs {
            name,
            bucket_name,
            region,
            storage_prefix,
        } => storage::gcs::cmd_new_gcs(
            input,
            output_snd,
            xvc_root,
            name,
            bucket_name,
            region,
            storage_prefix,
        ),
        #[cfg(feature = "wasabi")]
        StorageNewSubCommand::Wasabi {
            name,
            bucket_name,
            endpoint,
            storage_prefix,
        } => storage::wasabi::cmd_new_wasabi(
            output_snd,
            xvc_root,
            name,
            bucket_name,
            endpoint,
            storage_prefix,
        ),
        StorageNewSubCommand::Rsync {
            name,
            host,
            port,
            user,
            storage_dir,
        } => {
            storage::rsync::cmd_new_rsync(output_snd, xvc_root, name, host, port, user, storage_dir)
        }
    }
}

/// Removes a storage from the configurations.
///
/// This doesn't remove the history associated with them.
fn cmd_storage_remove(
    _input: std::io::StdinLock,
    _output_snd: &XvcOutputSender,
    _xvc_root: &XvcRoot,
    _name: String,
) -> Result<()> {
    todo!()
}

/// Lists all available storages.
///
/// It runs [XvcStorage::display] and lists all elements line by line to
/// `output_snd`.
fn cmd_storage_list(
    _input: std::io::StdinLock,
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
) -> Result<()> {
    let store: XvcStore<XvcStorage> = xvc_root.load_store()?;

    for (_, s) in store.iter() {
        output!(output_snd, "{}\n", s);
    }

    Ok(())
}
