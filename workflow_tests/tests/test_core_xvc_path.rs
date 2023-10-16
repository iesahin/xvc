mod common;
use std::path::Path;

use common::*;

use proptest::prelude::*;
use xvc_core::XvcPath;

#[test]
fn test_xvc_path_naming() -> xvc::Result<()> {
    test_logging(log::LevelFilter::Trace);

    let xvc_root = common::run_in_temp_xvc_dir()?;
    proptest!(|(current_dir in ".*", path in ".+")| {
        prop_assume!(path != "/");
        prop_assume!(!Path::new(&current_dir).is_absolute());
        let current_dir = xvc_root.absolute_path().join(current_dir);
        let path = Path::new(&path);
        prop_assume!(path.is_relative() || path.starts_with(&current_dir));
        let xvc_path = XvcPath::new(&xvc_root, &current_dir, path).unwrap();
        // xvc_path shouldn't end with "/"
        prop_assert!(!xvc_path.to_string().ends_with('/'));
    });

    clean_up(&xvc_root)
}
