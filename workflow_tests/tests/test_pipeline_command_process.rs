mod common;

use std::time::Duration;

use common::*;
use xvc_pipeline::{CommandProcess, XvcStep, XvcStepCommand};

use xvc::error::Result;

use xvc_test_helper::{create_directory_tree, test_logging};
use xvc_tests::watch;

#[test]
fn test_pipeline_command_process() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let xvc_root = run_in_temp_xvc_dir()?;

    let n_dirs = 100;
    let n_files = 100;

    create_directory_tree(&xvc_root, n_dirs, n_files, 1, None)?;
    watch!(xvc_root);

    let mut cp = CommandProcess::new(
        &XvcStep {
            name: "test".to_owned(),
        },
        &XvcStepCommand {
            command: "find . -name '*.bin'".to_owned(),
        },
    );

    watch!(cp);

    cp.run()?;

    watch!(cp);

    cp.update_output_channels()?;

    watch!(cp);

    let mut n_total: usize = 0;

    while let Ok(l) = cp.stdout_receiver.recv_timeout(Duration::from_millis(10)) {
        n_total += l.split('\n').count();
        assert!(l.contains(".bin"));
        watch!(l);
    }

    watch!(n_total);
    assert_eq!(n_total, n_dirs * n_files + 1);

    Ok(())
}
