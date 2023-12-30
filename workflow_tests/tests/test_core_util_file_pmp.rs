mod common;
use common::*;
use fs_extra::dir::create;
use log::LevelFilter;
use xvc_core::{
    util::xvcignore::{walk_parallel, walk_serial},
    XvcPath, XvcPathMetadataProvider,
};

use std::{fs::remove_file, path::Path, time::Duration};

use xvc_logging::watch;

use xvc::error::Result;

#[test]
fn test_pmp() -> Result<()> {
    test_logging(LevelFilter::Trace);
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let xvc_root = run_in_temp_xvc_dir()?;
    watch!(xvc_root);

    // We create this directory tree BEFORE the pmp so that we'll assert that it won't be notified
    // and won't collect these.
    let n_dirs = 3;
    let n_files = 2;
    create_directory_tree(&xvc_root, n_dirs, n_files, 10, None)?;

    let pmp = XvcPathMetadataProvider::new(&output_sender, &xvc_root)?;
    let initial_path_map = pmp.current_path_metadata_map_clone()?;

    // Test skip list
    for skipped in [".dvc", ".xvc", ".git"] {
        let xp = XvcPath::new(&xvc_root, xvc_root.absolute_path(), Path::new(skipped))?;
        assert!(
            !initial_path_map.contains_key(&xp),
            "Result Contains {:?}",
            xp
        )
    }

    // Test create / update / delete in the background
    let fn1 = "file1.bin";
    let path1 = xvc_root.absolute_path().join(fn1);
    let xpath1 = XvcPath::new(&xvc_root, &xvc_root, &path1)?;
    let orig_size = 100;
    generate_random_file(&path1, orig_size, None);
    let md1 = pmp.get(&xpath1);
    assert!(md1.is_some());
    assert!(md1.unwrap().is_file());
    assert!(md1.unwrap().size == Some(orig_size as u64));

    let new_size = 200;
    generate_random_file(&path1, new_size, None);
    let md1 = pmp.get(&xpath1);
    assert!(md1.is_some());
    assert!(md1.unwrap().is_file());
    assert!(md1.unwrap().size == Some(new_size as u64), "{:?}", md1);

    remove_file(&path1)?;

    assert!(!pmp.path_present(&xpath1));

    let glob_paths = pmp.glob_paths("**/*.bin")?;
    assert!(glob_paths.len() == n_dirs * n_files);

    remove_file(Path::new("dir-0001/file-0001.bin"))?;

    let glob_paths = pmp.glob_paths("**/*.bin")?;
    assert!(glob_paths.len() == n_dirs * n_files - 1);

    clean_up(&xvc_root)
}
