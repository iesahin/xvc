mod common;
use common::*;

use proptest::prelude::*;
use xvc_core::XvcPath;
use xvc_walker::AbsolutePath;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, time};
use xvc::core::types::*;
use xvcfiletype::XvcFileType;
use xvcmetadata::XvcMetadata;

proptest! {

#[test]
fn test_xvc_file_metadata(filesize in 0..100000u64, filestr in "file-[0-9][0-9][0-9][0-9]\\.bin") {

    test_logging(log::LevelFilter::Trace);

    // setup::logging(LevelFilter::Trace);
    let dir = common::run_in_temp_dir();

    let filename = Path::new(&filestr);
    common::generate_random_file(&filename, filesize as usize);
    // Wait the file to be written, the test fails sometimes for this
    sleep(Duration::from_millis(10));
    let file_md = filename.metadata()?;
    let xvc_md_1 = XvcMetadata::from(file_md);
    prop_assert!(xvc_md_1.size == Some(filesize));
    prop_assert!(xvc_md_1.file_type == XvcFileType::File);
    let duration = time::SystemTime::now()
        .duration_since(xvc_md_1.modified.unwrap())
        .unwrap();
    prop_assert!(duration < Duration::from_secs(2));

    fs::remove_dir_all(&dir).unwrap();
}


}

#[test]
fn test_xvc_path_naming() -> xvc::Result<()> {
    test_logging(log::LevelFilter::Trace);

    let xvc_root = common::run_in_temp_xvc_dir()?;
    proptest!(|(current_dir in ".*", path in ".+")| {
        prop_assume!(path != "/");
        prop_assume!(!Path::new(&current_dir).is_absolute());
        let current_dir = AbsolutePath::from(xvc_root.absolute_path().join(current_dir));
        let path = Path::new(&path);
        prop_assume!(path.is_relative() || path.starts_with(&current_dir));
        let xvc_path = XvcPath::new(&xvc_root, &current_dir, &path).unwrap();
        // xvc_path shouldn't end with "/"
        prop_assert!(!xvc_path.to_string().ends_with("/"));
    });

    clean_up(&xvc_root)
}
