use crate::error::{Error, Result};
use clap::Parser;
use crossbeam_channel::{unbounded, Sender};
use log::warn;
use std::{env, path::PathBuf};
use xvc_config::{FromConfigKey, UpdateFromXvcConfig, XvcConfig, XvcConfigInitParams};
use xvc_core::{
    util::file::{path_metadata_channel, pipe_filter_path_errors},
    HashAlgorithm, XvcDigest, XvcRoot,
};
use xvc_logging::{watch, XvcOutputLine};
use xvc_walker::AbsolutePath;

use crate::common::{calc_digest, pipe_path_digest};

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[clap(version, author)]
/// Calculate hash of given files
///
/// Note that this doesn't use .xvcignore facility and doesn't require an xvc root. It loads the
/// configuration from xvc repository if it runs within, otherwise uses user, system or default
/// options.
pub struct HashCLI {
    #[clap(short, long)]
    /// Algorithm to calculate the hash. One of blake3, blake2, sha2, sha3. All algorithm variants produce
    /// 32-bytes digest.
    algorithm: Option<HashAlgorithm>,
    #[clap(long)]
    /// Consider the file as a text file. Otherwise uses [is_text_file] function to decide.
    text_file: bool,
    /// Files to process
    targets: Vec<PathBuf>,
}

impl UpdateFromXvcConfig for HashCLI {
    fn update_from_conf(self, conf: &XvcConfig) -> xvc_config::error::Result<Box<Self>> {
        let algorithm = self
            .algorithm
            .unwrap_or_else(|| HashAlgorithm::from_conf(conf));
        Ok(Box::new(Self {
            algorithm: Some(algorithm),
            text_file: self.text_file,
            targets: self.targets.clone(),
        }))
    }
}

pub fn cmd_hash(
    output_snd: Sender<XvcOutputLine>,
    xvc_root: Option<&XvcRoot>,
    opts: HashCLI,
) -> Result<()> {
    let conf = match xvc_root {
        Some(xvc_root) => xvc_root.config().clone(),
        None => XvcConfig::new(XvcConfigInitParams {
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

    let text_file = opts.text_file;
    let targets = opts.targets;
    let send_output = |path: PathBuf, digest: XvcDigest| {
        output_snd
            .send(XvcOutputLine::Output(format!(
                "{}\t{}",
                digest,
                path.to_string_lossy()
            )))
            .unwrap();
    };

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
            pipe_path_digest(filtered_path_rec, digest_snd, &algorithm, text_file)?;

            for (path, digest) in digest_rec {
                watch!(path);
                watch!(digest);
                send_output(path, digest);
            }
        } else if t.is_file() {
            let digest = calc_digest(&t, &algorithm, text_file)?;
            send_output(t, digest);
        } else {
            warn!("Unsupported FS Type: {:?}", t);
        }
    }

    Ok(())
}
