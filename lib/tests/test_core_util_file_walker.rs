mod common;
use common::*;
use log::LevelFilter;
use xvc_core::{
    util::xvcignore::{walk_parallel, walk_serial},
    XvcPath,
};
use xvc_test_helper::test_logging;

use std::{path::Path, time::Duration};

use xvc_logging::watch;

use xvc::error::Result;

#[test]
fn test_walk() -> Result<()> {
    test_logging(LevelFilter::Trace);
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let xvc_root = run_in_example_xvc(true)?;
    watch!(xvc_root);

    let (pmp1, _) = walk_serial(&output_sender, &xvc_root, true)?;

    assert!(!pmp1.is_empty());

    // Test skip list
    for skipped in [".dvc", ".xvc", ".git"] {
        let xp = XvcPath::new(&xvc_root, xvc_root.absolute_path(), Path::new(skipped))?;
        assert!(!pmp1.contains_key(&xp), "Result Contains {:?}", xp)
    }

    let (pmp2, _) = walk_parallel(&xvc_root, true)?;

    watch!(pmp1);
    watch!(pmp2);

    let mut diff1 = Vec::<&XvcPath>::new();
    for k in pmp1.keys() {
        if !pmp2.contains_key(k) {
            diff1.push(k);
        }
    }

    watch!(diff1);

    let mut diff2 = Vec::<&XvcPath>::new();
    for k in pmp2.keys() {
        if !pmp1.contains_key(k) {
            diff2.push(k);
        }
    }

    watch!(diff2);

    assert!(diff1.is_empty());
    assert!(diff2.is_empty());

    let mut output_lines = Vec::<String>::new();
    watch!(output_lines);
    while let Ok(Some(l)) = output_receiver.recv_timeout(Duration::from_secs(1)) {
        output_lines.push(l.to_string());
    }

    let output = output_lines.into_iter().collect::<Vec<String>>().join("\n");
    watch!(output);

    for (p, m) in pmp1 {
        assert!(pmp2[&p] == m)
    }

    clean_up(&xvc_root)
}
