//! The `xvc` API.

pub use crate::error::{Error, Result};

pub use xvc_config as config;
pub use xvc_core as core;
pub use xvc_ecs as ecs;
pub use xvc_file as file;
pub use xvc_logging as logging;
pub use xvc_pipeline as pipeline;
pub use xvc_storage as storage;

pub use xvc_core::debug;
pub use xvc_core::error;
pub use xvc_core::info;
pub use xvc_core::panic;
pub use xvc_core::trace;
pub use xvc_core::warn;
pub use xvc_core::watch;

pub use xvc_core::XvcConfig;
pub use xvc_core::XvcConfigOptionSource;
pub use xvc_core::XvcConfigParams;

pub use xvc_core::AbsolutePath;

pub use xvc_core::XvcRoot;
/// Commands usually receive an optional xvc_root object for the repository
pub type XvcRootOpt = Option<XvcRoot>;

pub use xvc_file::BringCLI as XvcFileBringCLI;
pub use xvc_file::CarryInCLI as XvcFileCarryInCLI;
pub use xvc_file::CopyCLI as XvcFileCopyCLI;
pub use xvc_file::HashCLI as XvcFileHashCLI;
pub use xvc_file::ListCLI as XvcFileListCLI;
pub use xvc_file::MoveCLI as XvcFileMoveCLI;
pub use xvc_file::RemoveCLI as XvcFileRemoveCLI;
pub use xvc_file::SendCLI as XvcFileSendCLI;
pub use xvc_file::TrackCLI as XvcFileTrackCLI;
pub use xvc_file::UntrackCLI as XvcFileUntrackCLI;
pub use xvc_file::XvcFileCLI;

pub use xvc_file::cmd_carry_in as file_carry_in;
pub use xvc_file::cmd_copy as file_copy;
pub use xvc_file::cmd_hash as file_hash;
pub use xvc_file::cmd_list as file_list;
pub use xvc_file::cmd_move as file_move;
pub use xvc_file::cmd_remove as file_remove;
pub use xvc_file::cmd_send as file_send;
pub use xvc_file::cmd_track as file_track;
pub use xvc_file::cmd_untrack as file_untrack;

pub use xvc_pipeline::cmd_dag as pipeline_dag;
pub use xvc_pipeline::cmd_delete as pipeline_delete;
pub use xvc_pipeline::cmd_export as pipeline_export;
pub use xvc_pipeline::cmd_import as pipeline_import;
pub use xvc_pipeline::cmd_list as pipeline_list;
pub use xvc_pipeline::cmd_new as pipeline_new;
pub use xvc_pipeline::cmd_run as pipeline_run;
pub use xvc_pipeline::cmd_update as pipeline_update;

pub use xvc_pipeline::cmd_step_dependency as pipeline_step_dependency;
pub use xvc_pipeline::cmd_step_new as pipeline_step_new;
pub use xvc_pipeline::cmd_step_output as pipeline_step_output;
pub use xvc_pipeline::cmd_step_show as pipeline_step_show;
pub use xvc_pipeline::cmd_step_update as pipeline_step_update;
