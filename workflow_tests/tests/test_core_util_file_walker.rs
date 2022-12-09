mod common;
use common::*;
use log::LevelFilter;
use xvc_core::{
    util::xvcignore::{walk_parallel, walk_serial},
    XvcPath,
};

use std::path::Path;

use xvc_logging::watch;

use xvc::error::Result;

#[test]
fn test_walk() -> Result<()> {
    let xvc_root = run_in_example_xvc(true)?;
    watch!(xvc_root);

    let (pmp1, _) = walk_serial(&xvc_root, true)?;

    assert!(pmp1.len() > 0);

    // Test skip list
    for skipped in vec![".dvc", ".xvc", ".git"] {
        let xp = XvcPath::new(&xvc_root, xvc_root.absolute_path(), &Path::new(skipped))?;
        assert!(!pmp1.contains_key(&xp), "Result Contains {:?}", xp)
    }

    common::test_logging(LevelFilter::Trace);
    let (pmp2, _) = walk_parallel(&xvc_root, true)?;

    let mut diff1 = Vec::<&XvcPath>::new();
    for k in pmp1.keys() {
        if !pmp2.contains_key(&k) {
            diff1.push(k);
        }
    }

    watch!(diff1);

    let mut diff2 = Vec::<&XvcPath>::new();
    for k in pmp2.keys() {
        if !pmp1.contains_key(&k) {
            diff2.push(k);
        }
    }

    watch!(diff2);

    assert!(diff1.len() == 0);
    assert!(diff2.len() == 0);

    for (p, m) in pmp1 {
        assert!(pmp2[&p] == m)
    }

    clean_up(&xvc_root)
}
