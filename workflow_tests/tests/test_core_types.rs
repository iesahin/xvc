mod common;
use common::*;

use proptest::prelude::*;

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
    common::generate_random_file(filename, filesize as usize, None);
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

    fs::remove_dir_all(dir).unwrap();
}


}
