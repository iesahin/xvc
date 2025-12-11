# Xvc Changelog

## Unreleased

- Refactored storage operations to use dynamic dispatch (`dyn XvcStorageOperations`), reducing boilerplate code.

### TODO: Documentation Updates Needed

- [ ] Document breaking change: Environment variable renamed from `XVC_STORAGE_SECRET_KEY_<storage_name>` to `XVC_STORAGE_SECRET_ACCESS_KEY_<storage_name>` for all S3-compatible storage backends (S3, Minio, R2, Wasabi, Digital Ocean, GCS). Users need to update their environment variable names. Add migration guide or backward compatibility note.
- [ ] Document test configuration change: CI now tests Wasabi instead of Digital Ocean storage (lib/Cargo.toml test features updated).
- [ ] Document `TempDir::keep()` behavior: Changed from `TempDir::new()?.into_path()` to `TempDir::new()?.keep()` in storage tests. This may affect temporary directory cleanup behavior in tests.
- [ ] Update documentation for uncommented test line in `run-tests.zsh` - verify this is intentional for CI execution.
- [ ] Verify and document updated dependencies including major version bumps (blake3, sha2, rayon, regex, uuid, cached, proptest, rand 0.9, etc.).

## v0.6.17 (2025-04-22)

- Added rclone storage option
- Added generic rclone storage tests
- Reexported xvc-logging, xvc-config, xvc-ecs functionality from xvc-core
- Removed lower level crate dependencies from xvc-file, xvc-pipelines, xvc-storage crates
- Added Readme files to config and ecs crates

## v0.6.16 (2025-03-22)

- PR: <https://github.com/iesahin/xvc/pull/270>
- Added xvc storage remove command

## v0.6.15 (2025-02-01)

- PR: <https://github.com/iesahin/xvc/pull/266>
- Fixed a bug preventing Xvc to be run outside of repos
- Added static nushell completions

## v0.6.14 (2025-01-29)

- PR: <https://github.com/iesahin/xvc/pull/265>
- Added command completions for subcommands and options
- Added dynamic completions for xvc_paths, storages, pipelines, pipeline steps, etc. 
- Renamed xvc pipeline dag --format dot to graphviz
- Added experimental homebrew tap

## v0.6.13 (2024-12-30)

- PR: <https://github.com/iesahin/xvc/pull/263>
- Removed reflink from default features
- Hide directories in xvc file list output by default and add --show-dirs option
- Set core.quotepath=off in git ls-files call to get paths in UTF-8 to match
- Handle missing files in xvc file bring more gracefully
- Fixed git version string to also consider lightweight tags
- Fixed xvc file remove bug that panics when content digests not found
- Fixed xvc file list help text and added a test/example for ignored files
- Added more targets to Github builds
- Releases now use houseabsolute/actions-rust-cross@v0

## 0.6.12 (2024-11-30)

- PR: <https://github.com/iesahin/xvc/pull/262>
- Add --include-git-files option to xvc file track and xvc file list commands
- Don't track and list Git-tracked files by default
- Add ListFormat::empty for default xvc file list format
- Expose types from `xvc::file::list` to be used in GUI
- Refactor `xvc file list` command handler for Xvc GUI
- Began to use dtolnay/rust-toolchain for Github Actions
- Began to use taiki-e/install-action@cargo-llvm-cov for codecov 
- Fixed cache permissions issue

## 0.6.11 (2024-09-04)

- PR: <https://github.com/iesahin/xvc/pull/260>
- Bump dependencies
- Replace globset with fast-glob for memory usage
- Remove --details option from xvc check-ignore
- Fixed xvc check-ignore to work with supplied paths
- Fixed loading store targets from xvc file list
- Directory targets in various commands doesn't require / at the end when they only exist in the cache
- Removed some duplicate tests from ignore
- Minio tests now use mc instead of s3cmd
- Add a step to run a subset of tests in CI for faster feedback

## 0.6.10 (2024-08-04)

- PR: <https://github.com/iesahin/xvc/pull/259>
- Removed caching for globs that caused bugs in Python bindings and long running processes.
- Documentation updates
- Fix optional features. Now inter-workspace dependencies are defined by `default-features = false` on Cargo.toml
- Added `bundled-openssl` feature to use `vendored` feature of `openssl` crate optionally. This is turned on for Windows builds on GA.
- Crates are published from Github Actions

## 0.6.9 (2024-07-29)

- Added sqlite-query dependency to xvc pipelines. A step is invalidated to run when the output from an SQLite query changes.
- Moved bundled sqlite behind a feature flag, bundled_sqlite.
- Merged xvc-workflow-tests crate to the main xvc crate.
- Added compiling Xvc with non-default features document

## 0.6.8 (2024-07-15)

- Modifications for Python bindings

## 0.6.7 (2024-03-18)

- Added `xvc file share` command to share files from S3 signed URLs

## 0.6.6 (2024-01-12)

- Added `xvc pipeline step remove` command
- Added `to` and `for` as aliases to `--step-name` option in `xvc pipeline step dependency`
- Added custom Display implementation for dependency types

## 0.6.5 (2024-01-01)

- Add benchmarks between Xvc and DVC

  - Add how-to/benchmark-versus-dvc.md
  - Add regex filtering for how-to documents to trycmd test docs

- Refactored Pipelines API to expose more functionality
  - PR: <https://github.com/iesahin/xvc/pull/243>
  - Exposed Git operations in the API
- Fixed `xvc pipeline step update` clobbering `--when` option

- Added `xvc pipeline step list` command to list pipeline steps

- Updated file related operations not to list all files
  - Added `XvcPathMetadataProvider` to `xvc-walker` to cache file system metadata
- Added git revision to the version stringZ
- `xvc file list` doesn't list files starting with `.` now.
  - Added `--show-dot-files` to list them.
- Fixed `--recheck-method` to be reset to default if not specified

## 0.6.3 (2023-11-02)

- Updated README and added its commands to the tests
- Updated openssl dependency to use vendored feature for Python bindings
- Fixed clippy warnings
- Initial Python bindings in <https://github.com/iesahin/xvc.py>

## 0.6.2 (2023-10-13)

- Updated `xvc file track` reference and examples
- Added mdbook-admonish for admonitions
- Fixed `--skip-git` option not working correctly
  - Added documentation for turning off automated Git operations
  - PR: <https://github.com/iesahin/xvc/pull/238>
- Updated Readme and added it to tests

## 0.6.1 (2023-10-05)

- Added --min-size (-s) option to xvc-test-helper create-directory-tree
  - PR: <https://github.com/iesahin/xvc/pull/229>
- Updated `xvc pipeline dag` and reference docs.
  - PR: <https://github.com/iesahin/xvc/pull/232>
  - Dependency and outputs are shown with different shapes according to their types in Graphviz format
  - Simplify DAG creation for both dot and mermaid formats

## 0.6.0 (2023-08-28)

- Major overhaul in pipelines
  - PR: <https://github.com/iesahin/xvc/pull/224/>
  - Added `--generic`, `--url`, `--regex-items`, `--line-items`, `--glob-items` dependencies
  - as dependency types to `xvc pipeline step dependency` command
  - Updated `xvc pipelines` to run the pipeline by creating a thread for each step.
  - Updated pipelines state machine
  - Added environment variable injection to `--regex-items`, `--line-items` and `--glob-items` dependencies.
  - Reference examples for `xvc pipeline export` and `xvc pipeline import`
  - Refactored `xvc-core` crate for digest structures
  - Refactored `xvc-pipelines` create to move dependencies to files
- Added reference examples for `xvc storage new ...` commands.
  - PR: <https://github.com/iesahin/xvc/pull/222>
  - Issue: <https://github.com/iesahin/xvc/issue/221>
- Add `xvc pipeline new` reference examples.

  - Remove `--set-default` option from the command.

- Added reference examples for `xvc storage new ...` commands.

  - PR: <https://github.com/iesahin/xvc/pull/222>
  - Issue: <https://github.com/iesahin/xvc/issue/221>

- Add `--format mermaid` to `xvc pipeline dag`

- Standardize digests with AttributeDigest trait
- Add Diffable trait to compare records and actuals
- Refactor `xvc pipeline` comparisons to use Diff

## 0.5.2 (2023-02-13)

- Refactor "cache type" to "recheck method" in all code and documentation
  - Issue: <https://github.com/iesahin/xvc/issues/203>
  - Renamed `CacheType` to `RecheckMethod`
  - Revised documentation for recheck methods
- Add `xvc file untrack` command.
  - Issue: <https://github.com/iesahin/xvc/issues/113>
  - Write the reference page: book/src/ref/xvc-file-untrack.md
    - I believe there must be two separate commands: `xvc file untrack` and `xvc file remove`. The former is to remove the file from Xvc tracking, and the latter is to remove the file from the workspace, cache, or storages. There are valid use cases to remove the file from cache without untracking it, and vice versa.
    - There will also be a `xvc file versions` command to list the versions of a file and restore them to a directory.
    - I think it's better to implement remove and untrack commands in one go.
    - Added `join` to AbsolutePath.
      - Assert the parameter is not absolute.
        - Caused error in xvc init: Fixed ✅
        - Caused error in updating gitignores: Fixed ✅
    - Error in recheck for existing paths: Fixed ✅
    - XvcCachePath returns a partial path for reporting
    - Change all PathBuf elements in XvcRoot to AbsolutePath
    - Sort results of `--restore-versions` by entity id ✅
    - Delete cache files
    - Remove empty directories after untracking ✅
  - Update book/src/start/from-dvc.md for `xvc file untrack` ✅
- Restructure output channel to send/receive `Option<XvcOutputLine>` instead of `XvcOutputLine`
  - Refactor all commands to use the new output channel ✅
- Add `xvc file remove` command
  - Add RemoveCLI and the command dispatcher ✅
  - Implemented remove for local cache
    - Implement `--all-versions`
    - Implement `--only-version`
  - Implement `--from-remote`
  - Implement `XvcStorageOperations::delete` for all storage types
- Update `xvc-test-helper generate-random-file` and `create-directory-tree` to receive random seed
- Update `xvc storage new local` ref page to include examples

## 0.5.1 (2023-01-28)

- Fix Build Badge in Readme
  - Fixed per <https://github.com/badges/shields/issues/8671>
- Revised Readme and added _Future and Maintanence_ section.
- Fix EntityGenerator bug that saves even if no new entity is created
  - Issue: <https://github.com/iesahin/xvc/issues/185>
  - Added test_multi_save
  - Added a dirty flag to EntityGenerator
  - Separated load and new behavior
    - New makes the generator dirty. Load doesn't.
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
    - Wrote modified functions of operations and destination mapping from cmd_copy
    - Other functions from cmd_copy are reused
  - Add template for xvc-file-move.in
  - Fixed a bug in `xvc file copy` about changed source files.
  - `xvc file move --no-recheck` deletes the source files.
- Added Giscus to the documentation
  - PR: <https://github.com/iesahin/xvc/pull/215>
  - All docs pages can be commented with Giscus.

## 0.5.0 (2023-01-23)

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
  - Issue: <https://github.com/iesahin/xvc/issues/179>
  - PR: <https://github.com/iesahin/xvc/issues/206>
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
              - If a directory is not already ignored in creation, we can create a gitignore with a single line `*` to ignore all files.
              - After all files are rechecked, we can check whether they are not ignored by Git, and update necessary gitignores.
        - Create an IgnoreWriter system with crossbeam_channels
          - The channel will send/receive `Option<IgnoreOperation>` messages.
          - If it receives a None message, it will stop and the collected
            dir/files will be written to ignore files.
          - This pattern can be used for all operations.
      - Split targets_from_store to receive a store struct to filter. ✅
        - This is to prevent unnecessary reload in copy.
      - Convert former XvcRoot type to XvcRootInner and XvcRoot to `Arc<XvcRootInner>`
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
      - I did this in the trait ✅
      - Modified all receive functions to return a temp dir ✅
