mod common;

use std::{fs, path::Path};

use xvc::core::util::*;

#[test]
fn test_is_text_file() {
    // setup::logging(LevelFilter::Trace);
    use file::is_text_file;
    let repo_dir = common::run_in_temp_dir();

    for i in 1..10 {
        let bin_fn = format!("test-{}.bin", i);
        let bin_p = Path::new(&bin_fn);
        common::generate_random_file(bin_p, 100000, None);
        assert!(
            !is_text_file(bin_p).unwrap(),
            "{:?} seems text file incorrectly",
            bin_p
        );
        let text_fn = format!("test-{}.txt", i);
        let text_p = Path::new(&text_fn);
        common::generate_random_text_file(text_p, 100);
        assert!(
            is_text_file(text_p).unwrap(),
            "{:?} seems binary file incorrectly",
            text_p
        )
    }

    fs::remove_dir_all(repo_dir).unwrap();
}
