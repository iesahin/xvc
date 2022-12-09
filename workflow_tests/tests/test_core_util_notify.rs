mod common;
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
    let xvc_root = run_in_example_xvc(true)?;
    test_logging(log::LevelFilter::Trace);
    let initial_rules = IgnoreRules::try_from_patterns(&xvc_root, COMMON_IGNORE_PATTERNS)?;
    let walk_options = WalkOptions {
        ignore_filename: Some(XVCIGNORE_FILENAME.to_string()),
        include_dirs: true,
    };
    let res_paths = Arc::new(Mutex::new(Vec::<XvcWalkerResult<PathMetadata>>::new()));
    let res_paths_copy = res_paths.clone();
    let mut initial_paths = Vec::<XvcWalkerResult<PathMetadata>>::new();
    let all_rules = walk_serial(initial_rules, &xvc_root, &walk_options, &mut initial_paths)?;
    watch!(all_rules);
    let (_watcher, receiver) = make_watcher(all_rules)?;

    const MAX_ERROR_COUNT: usize = 100;

    let handle = thread::spawn(move || {
        let mut_res_paths = res_paths.clone();
        watch!(mut_res_paths);
        let mut res_paths = mut_res_paths.lock().unwrap();
        watch!(res_paths);
        let mut err_counter = MAX_ERROR_COUNT;
        loop {
            let r = receiver.try_recv();
            watch!(r);
            if let Ok(pe) = r {
                err_counter = MAX_ERROR_COUNT;
                watch!(pe);
                match pe {
                    PathEvent::Create { path, metadata } => {
                        res_paths.push(Ok(PathMetadata { path, metadata }))
                    }
                    PathEvent::Update { path, metadata } => {
                        res_paths.retain(|pm| pm.as_ref().unwrap().path.clone() != path.clone());
                        res_paths.push(Ok(PathMetadata { path, metadata }));
                    }
                    PathEvent::Delete { path } => {
                        res_paths.retain(|pm| pm.as_ref().unwrap().path != path)
                    }
                }
            } else {
                if err_counter > 0 {
                    err_counter -= 1;
                    sleep(Duration::from_millis(100));
                } else {
                    break;
                }
            }
        }
    });
    let files: Vec<String> = (1..10).map(|n| format!("file-000{n}.bin")).collect();
    watch!(files);
    let size1 = 10;
    files
        .iter()
        .for_each(|f| generate_random_file(&xvc_root.join(&PathBuf::from(f)), size1));

    sleep(Duration::from_millis(1000));

    let pmp_names: Vec<String> = res_paths_copy
        .clone()
        .lock()
        .unwrap()
        .iter()
        .map(|p| {
            p.as_ref()
                .unwrap()
                .path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();

    watch!(pmp_names);

    sleep(Duration::from_millis(1000));

    assert!(pmp_names
        .iter()
        .any(|f| f.to_string() == "file-0001.bin".to_string()));

    handle.join().unwrap();

    Ok(())
}
