//! Various utility functions
pub mod file;
pub mod git;
pub mod serde;
pub mod store;
pub mod xvcignore;

use std::fmt::Display;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

use crossbeam_channel::{bounded, Receiver};
use glob::glob;
use xvc_logging::watch;

use crate::error::{Error, Result};
use crate::CHANNEL_BOUND;

/// Expands the given glob `targets` to paths under `current_dir`.
/// It uses [glob::glob] to travers and expand the paths.
/// WARNING:
///     This function doesn't consider any ignore rules in traversal.
///     It may be better to use `xvc_walker::walk_parallel` first and
///     [crate::util::file::glob_paths] to filter the paths.
pub fn expand_globs_to_paths<T>(current_dir: &Path, targets: &[T]) -> Result<Vec<PathBuf>>
where
    T: AsRef<str> + Display,
{
    let current_dir = current_dir.to_str().ok_or(Error::UnicodeError {
        cause: current_dir.as_os_str().to_os_string(),
    })?;
    let mut paths = Vec::<PathBuf>::new();
    for t in targets {
        let glob_t = format!("{}/{}", current_dir, t);
        match glob(&glob_t) {
            Ok(glob_path_it) => {
                for p in glob_path_it {
                    match p {
                        Ok(path) => paths.push(path),
                        Err(source) => {
                            Error::GlobError { source }.error();
                        }
                    }
                }
            }
            Err(source) => return Err(Error::GlobPatternError { source }.error()),
        }
    }
    Ok(paths)
}

/// Converts stdin input to a channel.
///
/// It works by creating a thread inside.
///
/// WARNING:
///     Stdin is normally blocking the thread.
///     The thread may continue to live if there is no input in stdin.
///     A future version may also return the thread handle to prevent this.
pub fn stdin_channel() -> Receiver<String> {
    let (input_snd, input_rec) = bounded::<String>(CHANNEL_BOUND);
    crossbeam::scope(|s| {
        s.spawn(move |_| -> Result<()> {
            let stdin = io::stdin();
            loop {
                if input_snd.try_send("".into()).is_err() {
                    break;
                }
                let mut input = stdin.lock();
                watch!(&input);
                let mut buf = Vec::<u8>::new();
                watch!(&buf);
                let read_bytes = input.read(&mut buf)?;
                watch!(&buf);
                if read_bytes > 0 {
                    let s = String::from_utf8(buf);
                    input_snd
                        .send(s.unwrap_or_else(|_| "[ERROR] Requires UTF-8 Input".into()))
                        .unwrap();
                }
                drop(input);
                sleep(Duration::from_millis(10));

                watch!("Input looping");
            }

            watch!("Exit input loop");
            Ok(())
        });
    })
    .unwrap();

    input_rec
}
