//! Various utility functions
pub mod file;
pub mod git;
pub mod pmp;
pub mod serde;
pub mod store;
pub mod xvcignore;

use std::collections::HashMap;
use std::io::{self, Read};
use std::thread::sleep;
use std::time::Duration;

use crossbeam_channel::{bounded, Receiver};

use crate::error::Result;
use crate::{XvcMetadata, XvcPath, CHANNEL_BOUND};

/// A hashmap to store [XvcMetadata] for [XvcPath]
pub type XvcPathMetadataMap = HashMap<XvcPath, XvcMetadata>;

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
                let mut buf = Vec::<u8>::new();
                let read_bytes = input.read(&mut buf)?;
                if read_bytes > 0 {
                    let s = String::from_utf8(buf);
                    input_snd
                        .send(s.unwrap_or_else(|_| "[ERROR] Requires UTF-8 Input".into()))
                        .unwrap();
                }
                drop(input);
                sleep(Duration::from_millis(10));
            }

            Ok(())
        });
    })
    .unwrap();

    input_rec
}
