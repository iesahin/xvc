mod common;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::{FileTouch, FileWriteBin, PathChild};
use assert_fs::TempDir;
use common::*;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;

use common::*;
use xvc::error::Result;

use xvc::watch;
use xvc_core::util::xvcignore::COMMON_IGNORE_PATTERNS;
use xvc_core::XVCIGNORE_FILENAME;
use xvc_walker::notify::{make_watcher, PathEvent};
use xvc_walker::{walk_serial, IgnoreRules, PathMetadata, Result as XvcWalkerResult, WalkOptions};

#[test]
fn test_notify() -> Result<()> {
    let temp_dir = TempDir::new()?;
    env::set_current_dir(&temp_dir)?;
    watch!(temp_dir);
    test_logging(log::LevelFilter::Trace);
    let initial_rules = IgnoreRules::try_from_patterns(&temp_dir, COMMON_IGNORE_PATTERNS)?;
    let walk_options = WalkOptions {
        ignore_filename: Some(XVCIGNORE_FILENAME.to_string()),
        include_dirs: true,
    };
    let created_paths = Arc::new(RwLock::new(Vec::<PathBuf>::new()));
    let updated_paths = Arc::new(RwLock::new(Vec::<PathBuf>::new()));
    let deleted_paths = Arc::new(RwLock::new(Vec::<PathBuf>::new()));
    let created_paths_clone = created_paths.clone();
    let updated_paths_clone = updated_paths.clone();
    let deleted_paths_clone = deleted_paths.clone();
    let mut initial_paths = Vec::<XvcWalkerResult<PathMetadata>>::new();
    let files: Vec<ChildPath> = (1..5)
        .map(|n| temp_dir.child(format!("file-000{n}.bin")))
        .collect();

    let files_len = files.len();

    let all_rules = walk_serial(initial_rules, &temp_dir, &walk_options, &mut initial_paths)?;
    watch!(all_rules);
    let (watcher, receiver) = make_watcher(all_rules)?;
    watch!(watcher);

    watch!(initial_paths.len());

    watch!(files.len());

    const MAX_ERROR_COUNT: usize = 100;

    let event_handler = thread::spawn(move || {
        let mut err_counter = MAX_ERROR_COUNT;
        loop {
            watch!(receiver);
            let r = receiver.try_recv();
            watch!(r);
            if let Ok(pe) = r {
                err_counter = MAX_ERROR_COUNT;
                match pe {
                    PathEvent::Create { path, metadata } => {
                        let mut created_paths = created_paths.write().unwrap();
                        created_paths.push(path);
                    }
                    PathEvent::Update { path, metadata } => {
                        let mut updated_paths = updated_paths.write().unwrap();
                        updated_paths.push(path);
                    }
                    PathEvent::Delete { path } => {
                        let mut deleted_paths = deleted_paths.write().unwrap();
                        deleted_paths.push(path);
                    }
                }
            } else {
                if err_counter > 0 {
                    err_counter -= 1;
                } else {
                    break;
                }
            }

            watch!(err_counter);
            sleep(Duration::from_millis(
                ((MAX_ERROR_COUNT - err_counter) * 1) as u64,
            ));
        }
        watch!(err_counter);
        watch!(receiver);
        drop(receiver);
    });

    let file_modifier = thread::spawn(move || {
        sleep(Duration::from_millis(100));
        files.iter().for_each(|f| {
            watch!(f.path());
            f.touch().unwrap();
        });

        let size_updated = 20;

        files.iter().for_each(|f| {
            watch!(f.path());
            f.write_binary(&vec![0; size_updated]).unwrap();
        });

        files.iter().for_each(|f| {
            watch!(f.path());
            f.remove().unwrap();
        });

        sleep(Duration::from_millis(100));
        watch!(created_paths_clone);
        let created_paths = created_paths_clone.try_read().unwrap();
        watch!(created_paths);

        watch!(updated_paths_clone);
        let updated_paths = updated_paths_clone.try_read().unwrap();
        watch!(updated_paths);

        watch!(deleted_paths_clone);
        let deleted_paths = deleted_paths_clone.try_read().unwrap();
        watch!(deleted_paths);

        (
            created_paths.len(),
            updated_paths.len(),
            deleted_paths.len(),
        )
    });

    event_handler.join().unwrap();
    let (created_len, updated_len, deleted_len) = file_modifier.join().unwrap();
    assert!(created_len == files_len);
    assert!(updated_len == files_len);
    assert!(deleted_len == files_len);
    drop(files);
    Ok(())
}
