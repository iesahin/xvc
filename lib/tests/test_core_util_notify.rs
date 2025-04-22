mod common;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::{FileTouch, FileWriteBin, PathChild};
use assert_fs::TempDir;
use common::*;
use std::env;
use std::fs::remove_file;
use std::path::PathBuf;
use std::thread::{self, sleep};
use std::time::Duration;

use xvc::error::Result;

use xvc_core::util::xvcignore::COMMON_IGNORE_PATTERNS;
use xvc_core::XVCIGNORE_FILENAME;

use xvc_core::{make_polling_watcher, PathEvent};
use xvc_core::{walk_serial, WalkOptions};

#[test]
fn test_notify() -> Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;
    test_logging(log::LevelFilter::Trace);
    let walk_options = WalkOptions {
        ignore_filename: Some(XVCIGNORE_FILENAME.to_owned()),
        include_dirs: true,
    };
    let (created_paths_snd, created_paths_rec) = crossbeam_channel::unbounded();
    let (updated_paths_snd, _) = crossbeam_channel::unbounded();
    let (deleted_paths_snd, deleted_paths_rec) = crossbeam_channel::unbounded();

    let files: Vec<ChildPath> = (1..10)
        .map(|n| temp_dir.child(format!("file-000{n}.bin")))
        .collect();

    let files_len = files.len();

    files.iter().for_each(|f| {
        f.touch().unwrap();
    });
    const MAX_ERROR_COUNT: usize = 10;

    let (output_sender, output_receiver) = crossbeam_channel::unbounded();

    let (_initial_paths, all_rules) = walk_serial(
        &output_sender,
        COMMON_IGNORE_PATTERNS,
        &temp_dir,
        &walk_options,
    )?;
    assert!(output_receiver.is_empty());
    let (_watcher, receiver) = make_polling_watcher(all_rules)?;

    let event_handler = thread::spawn(move || {
        let mut err_counter = MAX_ERROR_COUNT;
        loop {
            let r = receiver.try_recv();
            if let Ok(Some(pe)) = r {
                err_counter = MAX_ERROR_COUNT;
                match pe {
                    PathEvent::Create { path, .. } => {
                        created_paths_snd.send(path).unwrap();
                    }
                    PathEvent::Update { path, .. } => {
                        updated_paths_snd.send(path).unwrap();
                    }
                    PathEvent::Delete { path } => {
                        deleted_paths_snd.send(path).unwrap();
                    }
                }
            } else if err_counter > 0 {
                err_counter -= 1;
            } else {
                break;
            }

            sleep(Duration::from_millis(
                ((MAX_ERROR_COUNT - err_counter) * 100) as u64,
            ));
        }
    });

    sleep(Duration::from_millis(3000));
    let size_updated = 20;

    files.iter().for_each(|f| {
        f.write_binary(&vec![0; size_updated]).unwrap();
    });

    sleep(Duration::from_millis(3000));
    files.iter().for_each(|f| {
        remove_file(f.path()).unwrap();
    });

    sleep(Duration::from_millis(2000));

    event_handler.join().unwrap();

    {
        let created_paths = created_paths_rec.iter().collect::<Vec<PathBuf>>();
        let len = created_paths.len();
        // This also gets the directory creation event itself
        assert!(len == files_len + 1);
    }
    // TODO: This is not working for some reason on macOS
    // {
    //     let updated_paths = updated_paths_rec.iter().collect::<Vec<PathBuf>>();
    //     assert_eq!(updated_paths.len(), files_len);
    // }

    {
        let deleted_paths = deleted_paths_rec.iter().collect::<Vec<PathBuf>>();
        assert_eq!(deleted_paths.len(), files_len);
    }

    Ok(())
}
