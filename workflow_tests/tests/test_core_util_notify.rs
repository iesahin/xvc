mod common;
use common::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
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
    let temp_dir = run_in_temp_dir();
    test_logging(log::LevelFilter::Trace);
    let initial_rules = IgnoreRules::try_from_patterns(&temp_dir, COMMON_IGNORE_PATTERNS)?;
    let walk_options = WalkOptions {
        ignore_filename: Some(XVCIGNORE_FILENAME.to_string()),
        include_dirs: true,
    };
    let created_paths = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    let updated_paths = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    let deleted_paths = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    let created_paths_clone = created_paths.clone();
    let updated_paths_clone = updated_paths.clone();
    let deleted_paths_clone = deleted_paths.clone();
    let mut initial_paths = Vec::<XvcWalkerResult<PathMetadata>>::new();
    let all_rules = walk_serial(initial_rules, &temp_dir, &walk_options, &mut initial_paths)?;
    watch!(all_rules);
    let (_watcher, receiver) = make_watcher(all_rules)?;

    watch!(initial_paths.len());

    const MAX_ERROR_COUNT: usize = 100;

    let handle = thread::spawn(move || {
        let mut err_counter = MAX_ERROR_COUNT;
        loop {
            let r = receiver.try_recv();
            if let Ok(pe) = r {
                err_counter = MAX_ERROR_COUNT;
                match pe {
                    PathEvent::Create { path, metadata } => {
                        let mut created_paths = created_paths.lock().unwrap();
                        created_paths.push(path);
                    }
                    PathEvent::Update { path, metadata } => {
                        let mut updated_paths = updated_paths.lock().unwrap();
                        updated_paths.push(path);
                    }
                    PathEvent::Delete { path } => {
                        let mut deleted_paths = deleted_paths.lock().unwrap();
                        deleted_paths.push(path);
                    }
                }
            } else {
                if err_counter > 0 {
                    err_counter -= 1;
                    watch!(err_counter);
                    sleep(Duration::from_millis(10));
                } else {
                    break;
                }
            }
        }
        drop(receiver);
    });

    let files: Vec<PathBuf> = (1..10)
        .map(|n| temp_dir.join(&PathBuf::from(format!("file-000{n}.bin"))))
        .collect();
    watch!(files.len());
    let size_created = 10;
    files
        .iter()
        .for_each(|f| generate_random_file(&f, size_created));

    sleep(Duration::from_millis(100));

    let size_updated = 20;

    files
        .iter()
        .for_each(|f| generate_random_file(&f, size_updated));

    sleep(Duration::from_millis(100));

    files.iter().for_each(|f| std::fs::remove_file(f).unwrap());

    sleep(Duration::from_millis(100));

    let created_paths = created_paths_clone.lock().unwrap();
    assert!(created_paths.len() == files.len());

    let updated_paths = updated_paths_clone.lock().unwrap();
    assert!(updated_paths.len() == files.len());

    let deleted_paths = deleted_paths_clone.lock().unwrap();
    assert!(deleted_paths.len() == files.len());

    handle.join().unwrap();
    clean_up_path_buf(temp_dir)
}
