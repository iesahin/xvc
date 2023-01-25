# Introduction

This document is a change log that I write for the project as I develop. It's a
tree, and subtasks are marked with indentation.

## Unreleased

- Fix Build Badge in Readme
  - Fixed per <https://github.com/badges/shields/issues/8671>
- Revised Readme and added _Future and Maintanence_ section.
- Fix EntityGenerator bug that saves even if no new entity is created
  - Issue: <https://github.com/iesahin/xvc/issues/185>
  - Added test_multi_save
  - Added a dirty flag to EntityGenerator
  - Separated load and new behavior
- Add `xvc file move` command.
  - Issue: <https://github.com/iesahin/xvc/issues/177>
  - Write the reference page: book/src/ref/xvc-file-move.md
  - Add `move` to `XvcFileCLI`
  - Add MoveCLI
  - Refactor and split cmd_copy
    - A function to get source files
    - A function to map source files to destination files
    - A function to perform copy operations
    - A function to recheck
  - Write cmd_move
    - Wrote a modified version operations and destination mapping from cmd_copy
    - Other functions from cmd_copy are reused

## 0.5.0 (2021-09-23)

- Refactor XvcEntity to `(u64, u64)`
  - Issue: <https://github.com/iesahin/xvc/issues/198>
  - PR: <https://github.com/iesahin/xvc/pulls/201>
  - `From<u128>` and `Into<u128>`
  - `From<(u64, u64)>` and `Into<(u64, u64)>`
  - Tests
    - Add tests for `From<u128>` and `Into<u128>` ecs/src/ecs/mod.rs
    - Fix doc tests that use `100usize` to create `XvcEntity`
  - Update the ECS documentation
    - Update arch/ecs.md
    - Search for any `XvcEntity` references that may be changed
- `xvc-test-helper` binary is not produced at builds
  - Moved it from dev-dependencies to dependencies in workflow_tests/Cargo.toml
    - Still doesn't work 🛑
    - We need binary dependencies in cargo: <https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html>,
    - It's available in nightly: <https://github.com/rust-lang/cargo/issues/9096>
    - Revert to dev-dependencies
  - `z_test_docs` fails immediately if no `xvc-test-helper` binary is found.
  - Run the tests without `-p workflow_tests`
    - Hypothesis: The reason the test helper binary is not produced is that we run only `workflow_tests` crate.
    - Looks this hypothesis is not correct.
  - The best way seems to be adding
    <https://docs.rs/escargot/latest/escargot/> and building the binary before
    the doc tests.
    - Now builds the binary before running the doc tests. ✅
- Write pipelines code documentation <https://github.com/iesahin/xvc/issues/88>
- Add `xvc file copy` command
  - Issue: https://github.com/iesahin/xvc/issues/179
  - PR: https://github.com/iesahin/xvc/issues/206
  - Create the user interface
    - Add `copy` to `XvcFileCLI`
    - Created CopyCLI
  - Write the documentation and doc tests:
    - Write initial examples: book/src/ref/xvc-file-copy.md
    - Create a fixture directory `xvc-file-copy.in`
  - Implement the command
    - Select source
    - Select destination
      - Do we store directories with trailing / or not❓
        - Write tests for consistency
          - Added `test_xvc_path_naming` proptests and modified XvcPath
            constructor to accept absolute paths conditionally. ✅
        - We don't store directories with trailing / ℹ️
      - Create destination XvcPaths ✅
        - Add join function to XvcPath ✅
      - Create destination cache type, metadata, digest, text-or-binary ✅
      - Should we create destination directory records❓
        - It's better to create them to update gitignore files. ✅
      - Update gitignore files in destinations
        - Use update_dir_gitignore for new directories and update_file_gitignore for new files.
        - Move gitignore functions from track/mod.rs to common/gitignore.rs
          - Gitignore handling is actually a recheck sub-operation.
            - Git doesn't mind if we don't create anything in the workspace.
            - We should update gitignores in recheck, but how can we do that for
              directories that may contain non-tracked files❓
            - While creating files and parent directories we can update gitignores
              in the parent directories.
               - If a directory is not already ignored in creation, we can
                 create a gitignore with a single line `*` to ignore all files.
               - After all files are rechecked, we can check whether they are
                 not ignored by Git, and update necessary gitignores.
        - Create an IgnoreWriter system with crossbeam_channels
          - The channel will send/receive Option<IgnoreOperation> messages.
          - If it receives a None message, it will stop and the collected
            dir/files will be written to ignore files.
          - This pattern can be used for all operations.
      - Split targets_from_store to receive a store struct to filter. ✅
        - This is to prevent unnecessary reload in copy.
      - Convert former XvcRoot type to XvcRootInner and XvcRoot to Arc<XvcRootInner>
        - This is to pass the object to threads easily.
  - Updated default format string for `xvc file list`
    - Moved `name` block to the end of the format string ✅

## v0.4.2 (2023-01-17)

- `xvc file carry-in` <https://github.com/iesahin/xvc/issues/174>
  - PR <https://github.com/iesahin/xvc/pull/194>
    - `xvc file list` debugging <https://github.com/iesahin/xvc/issues/197>
      - Fixed slicing bug ✅
      - Recursive option
        - If not given all files including the ignored files will be reported.
          - Ignored files will be reported with file type `I`
      - Add `G` for as a file type for git-tracked files.
      - `DX         224 2022-12-31 08:21:11   dir-0001/dir-0001  rcd \n`
        - Fix `rcd` ✅
      - Count lines in the result
        - I think it's better to write all of this as a doc test
- create a `xvc-test-helper create-directory-hierarchy` command.
  - Add a main.rs to xvc-test-helper ✅
  - Add clap to parse CLI
    - Add subcommands ✅
      - create directory tree
      - random dir name --prefix str --seed u64
      - random temp dir --prefix str
      - seeded temp dir --seed u64
      - create temp dir
      - run in temp dir
      - run in temp git dir
      - create temp git dir
      - generate random file filename size
      - generate filled file filename size byte
      - generate random text file filename num_lines
    - Add to doc-tests
      - added with `cargo_bin!` ✅
      - began to add `xvc-file-list.md`
        - Open doc test results in a directory
          - Use neovim for this
      - It looks we need to update directory permissions in the cache too
        - updated move_to_cache function
      - fix recheck errors
        - it looks recheck doesn't check whether the file is changed before trying to checkout
        - do we use `--text-or-binary` option to update the file?
          - removed the option from help text ✅
        - I think we need a `DEBUG` level in XvcOutput for otherwise irrelevant information
          - Added debug option to XvcOutputLine
          - Changed all noisy output to debug! ✅
      - fix `carry-in` errors
        - updated outputs
        - there seems to be a bug to update the stores
        - add watches for several places.
        - the bug was about missing configuration keys.
          - it must warn/panic when the keys are not there.
            - all machinery is there, it must report error, but doesn't.
      - there seems to be a bug in xvc list output about cached/workspace sizes
        - yes, there was. fixed the summary. ✅
    - started moving `test_file_list.rs` to document test.
      - `--recheck-as` option must be introduced instead of `--cache-type`.
      - there is a bug in `track` when `--cache-type` is given. 🐛
        - pmm doesn't contain directory contents
          - fixed ✅
      - the sorting for timestamp and size are not working
        - fixed ✅
      - if a field is blank or None, it should print spaces.
        - Done for size and timestamp ✅
    - Why the cache size is empty when they are not reported
      - Fixed. Loads the rec content digests always now. ✅
    - We need more tests for other sorting options to increase coverage perhaps.
      - removed older tests and added only the sorting test to xvc file list wf tests
      - tests in ref md is larger than this file anyway.
    - Listing only the changed.
      - As a status command.
  - Fix `xvc file hash` tests
    - create directory tree needs an option to create random files or filled files
      - update all uses ✅
      - modify test helper to have this option ✅
  - Fix `xvc file list` tests
    - Fix counting and sorting tests ✅
  - Could we have file, line, function etc in panic! / error! macros?
    - Modified and did this ✅
  - Fix `xvc file recheck parallel` tests
    - There is a failing command, which one?
      - It looks like a plain recheck after hardlink
      - The target permissions should be removed
    - The bug seems to be in `xvc file track`
      - There is a gitignore bug
        - Fixed it by using the targets directly
    - The failure is in cleanup, about permissions.
      - Delete files and directories one by one
        - Deleted by shell ✅
  - Fix `xvc root`
    - `--debug` should only determine the xvc.log output
      - changed output in `run_xvc` fn ✅
  - Fix `xvc pipeline export` tests
    - There must be sorting in the output, as we changed the stores to HStore ✅
  - Fix `xvc pipeline import` tests
    - The same changes, ordering of elements changed ✅
  - Fix `xvc pipeline run` tests
    - The example repository again and again ✅
  - Fix `xvc storage generic fs` tests
    - Where is the actual error?
      - It was about removing the repos
  - Fix `xvc storage local` tests ✅
    - Cache operations from storages should be done on temp dir and _move to cache_ must be used for all
      - This is to keep permission operations correct
      - I did this in the trait  ✅
      - Modified all receive functions to return a temp dir ✅
