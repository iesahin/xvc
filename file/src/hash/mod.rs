//! Data structures and functions for `xvc file hash`.
//!
//! - [HashCLI] defines the command line options.
//! - [cmd_hash] is the entry point for the command.
use crate::error::{Error, Result};
use clap::Parser;
use crossbeam_channel::unbounded;
use log::warn;
use std::{env, path::PathBuf};
use xvc_config::{FromConfigKey, UpdateFromXvcConfig, XvcConfig, XvcConfigParams};
use xvc_core::ContentDigest;
use xvc_core::{
    util::file::{path_metadata_channel, pipe_filter_path_errors},
    HashAlgorithm, TextOrBinary, XvcRoot,
};
use xvc_logging::{output, watch, XvcOutputSender};
use xvc_walker::AbsolutePath;

use crate::common::pipe_path_digest;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version, author)]
/// Calculate hash of given files
///
/// Note that this doesn't use .xvcignore facility and doesn't require an xvc root. It loads the
/// configuration from xvc repository if it runs within, otherwise uses user, system or default
/// options.
pub struct HashCLI {
    /// Algorithm to calculate the hash. One of blake3, blake2, sha2, sha3. All algorithm variants produce
    /// 32-bytes digest.
    #[arg(short, long)]
    algorithm: Option<HashAlgorithm>,
    /// For "text" remove line endings before calculating the digest. Keep line endings if
    /// "binary". "auto" (default) detects the type by checking 0s in the first 8Kbytes, similar to
    /// Git.
    #[arg(long, default_value("auto"))]
    text_or_binary: TextOrBinary,

    /// Files to process
    #[arg()]
    targets: Vec<PathBuf>,
}

impl UpdateFromXvcConfig for HashCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let algorithm = self
            .algorithm
            .unwrap_or_else(|| HashAlgorithm::from_conf(conf));
        Ok(Box::new(Self {
            algorithm: Some(algorithm),
            text_or_binary: self.text_or_binary,
            targets: self.targets.clone(),
        }))
    }
}
/// Entry point for `xvc file hash`.
///
/// Calculate hash of given files in `opts.targets` and send to `output_snd`.
pub fn cmd_hash(
    output_snd: &XvcOutputSender,
    xvc_root: Option<&XvcRoot>,
    opts: HashCLI,
) -> Result<()> {
    let conf = match xvc_root {
        Some(xvc_root) => xvc_root.config().clone(),
        None => XvcConfig::new(XvcConfigParams {
            default_configuration: xvc_core::default_project_config(false),
            current_dir: AbsolutePath::from(env::current_dir()?),
            include_system_config: true,
            include_user_config: false,
            project_config_path: None,
            local_config_path: None,
            include_environment_config: true,
            command_line_config: None,
        })?,
    };

    let opts = opts.update_from_conf(&conf)?;
    let algorithm = opts.algorithm.unwrap_or(HashAlgorithm::Blake3);

    let text_or_binary = opts.text_or_binary;
    let targets = opts.targets;

    for t in targets {
        watch!(t);
        if !t.exists() {
            Error::FileNotFound { path: t }.error();
            continue;
        }
        if t.is_dir() {
            let (path_snd, path_rec) = unbounded();
            path_metadata_channel(path_snd, &t)?;
            let (filtered_path_snd, filtered_path_rec) = unbounded();
            pipe_filter_path_errors(path_rec, filtered_path_snd)?;
            let (digest_snd, digest_rec) = unbounded();
            pipe_path_digest(filtered_path_rec, digest_snd, algorithm, text_or_binary)?;

            for (path, digest) in digest_rec {
                watch!(path);
                watch!(digest);
                output!(output_snd, "{digest}\t{}", path.to_string_lossy());
            }
        } else if t.is_file() {
            let digest = ContentDigest::new(&t, algorithm, text_or_binary)?;
            output!(output_snd, "{digest}\t{}", t.to_string_lossy());
        } else {
            warn!("Unsupported FS Type: {:?}", t);
        }
    }

    Ok(())
}
