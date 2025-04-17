mod common;

use log::LevelFilter;

use xvc::error::Result;
use xvc_core::XvcVerbosity;

#[test]
fn test_storage_remove() -> Result<()> {
    common::test_logging(LevelFilter::Trace);
    let xvc_root = common::run_in_temp_xvc_dir()?;

    let x = |cmd: &[&str]| -> Result<String> {
        common::run_xvc(Some(&xvc_root), cmd, XvcVerbosity::Trace)
    };

    let storage_names = ["backup-1", "backup-2"];

    for name in storage_names {
        let storage_dir = common::random_temp_dir(Some(name));
        x(&[
            "storage",
            "new",
            "local",
            "--name",
            name,
            "--path",
            storage_dir.to_string_lossy().as_ref(),
        ])?;
    }

    let storage_list: Vec<String> = x(&["storage", "list"])?
        .lines()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect();
    assert!(
        storage_list.len() == storage_names.len(),
        "storage_list: {storage_list:?}"
    );

    let remove_output = x(&["storage", "remove", "--name", storage_names[0]])?;
    assert!(remove_output.contains(storage_names[0]));

    let storage_list: Vec<String> = x(&["storage", "list"])?
        .lines()
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        })
        .collect();

    assert!(
        storage_list.len() == storage_names.len() - 1,
        "remove_output: {remove_output:?}\n\nstorage_list: {storage_list:?}"
    );

    Ok(())
}
