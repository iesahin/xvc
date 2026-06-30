# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is Xvc

Xvc is a Git-based MLOps/data versioning CLI tool. It tracks large binary files with content-addressed storage, syncs them to cloud backends, and runs data pipelines that re-execute steps only when their dependencies change. The binary is `xvc`; subcommands are `file`, `pipeline`, `storage`, `init`, `root`, and `check-ignore`.

## Commands

```bash
# Build
cargo build                          # debug build
cargo build --release                # release build

# Run tests (all workspace crates)
cargo test --workspace               # all unit + integration tests
cargo test -p xvc --test test_file_track_serial   # single integration test file
cargo test -p xvc-core               # single crate

# Doc tests (macOS only, trycmd-based, reads from book/src/)
XVC_TRYCMD_TESTS=file cargo test -p xvc --test z_test_docs

# Cloud storage integration tests (require secrets)
cargo test --features test-ci -p xvc --test test_storage_new_minio

# Lint
cargo clippy --workspace
cargo fmt --all

# Coverage (requires cargo-llvm-cov)
LLVM_PROFILE_FILE="${TMPDIR}/xvc-%p-%m.profraw" CARGO_INCREMENTAL=0 \
  RUSTFLAGS="-Cinstrument-coverage" \
  cargo llvm-cov --features test-ci --lcov --output-path lcov.info -p xvc
```

### Test environment variables

- `XVC_TRYCMD_TESTS` — comma-separated list of doc sections to run: `core`, `file`, `pipeline`, `storage`, `intro`, `start`, `how-to`
- `TRYCMD=overwrite` — regenerate expected outputs in doc tests
- `TRYCMD=dump` — dump actual outputs without failing

## Architecture

### Workspace crates (dependency order)

```
xvc-logging → xvc-ecs → xvc-walker → xvc-config → xvc-core
                                                        ↓
                                          xvc-file  xvc-pipeline  xvc-storage
                                                        ↓
                                                    xvc (lib + binary)
```

- **`logging`** — `XvcOutputLine` enum, `XvcOutputSender` type alias, and macros (`output!`, `info!`, `warn!`, `error!`, `debug!`, `watch!`, `uwr!`, `uwo!`). All output goes through a `crossbeam_channel` — never write directly to stdout/stderr.
- **`ecs`** — Entity-Component System. `XvcEntity` is a `(u64, u64)` key. `XvcStore<T>` persists components via an append-only `EventLog` (JSON files in `.xvc/store/`). `HStore<T>` (HashMap) and `VStore<T>` (Vec) are ephemeral. Relationship stores: `R11Store`, `R1NStore`, `RMNStore`.
- **`walker`** — Parallel/serial directory walker with `.gitignore`-style ignore rules (`IgnoreRules`), file-system event watching, and `AbsolutePath` type.
- **`config`** — Cascading config from defaults → system → user → project (`.xvc/config.toml`) → local (`.xvc/config.local.toml`) → env vars (`XVC_*`) → CLI flags. `XvcConfiguration` holds the final merged config; `XvcOptionalConfiguration` holds partials.
- **`core`** — `XvcRoot` (an `Arc<XvcRootInner>`) is the single repository handle passed everywhere. Re-exports all common types from the crates above. Contains hash algorithms (BLAKE3 default), `XvcPath`, `RecheckMethod`, `Diff`/`DiffStore`, and Git integration utilities.
- **`file`** — `track`, `recheck`, `carry-in`, `copy`, `move`, `list`, `hash`, `bring`, `send`, `share`, `untrack` subcommands.
- **`pipeline`** — Pipeline steps, dependency types (file, glob, glob-items, lines, line-items, regex, regex-items, url, param, sqlite, generic, step), DAG execution via `petgraph`. Steps run in parallel when not interdependent.
- **`storage`** — Storage backends: local, S3/MinIO/R2/GCS/Wasabi/DigitalOcean (via `rust-s3`), rsync, rclone, generic (shell commands). Cloud features are Cargo feature-gated.
- **`test_helper`** — `create_directory_tree`, `generate_random_file`, `run_in_temp_dir`, `run_in_temp_git_dir`, `random_temp_dir`. Used from all integration tests.
- **`lib`** — The `xvc` crate: binary entrypoint (`main.rs`), CLI dispatch (`cli/mod.rs`), public API re-exports (`api.rs`), and all integration tests under `tests/`.

### CLI dispatch flow

`main.rs` → `XvcCLI::from_args_os` → `cli::dispatch` → `load_xvc_root` → `dispatch_with_root` → `command_matcher`. Each subcommand's `run`/`cmd_*` function receives `&XvcOutputSender` and `Option<&XvcRoot>`.

Output is collected in a `crossbeam_channel::bounded` channel and printed by a dedicated output thread. Verbosity is controlled by `-v`/`--quiet`/`--debug` flags on the root command.

### ECS persistence pattern

Every stored type must implement `Storable`, typically via the `persist!` macro:

```rust
persist!(MyType, "my-type");  // generates Storable impl; "my-type" becomes the store directory name
```

`.xvc/store/` contains subdirectories per component type. Each subdirectory holds timestamped JSON event-log files. On load, all files are replayed in sorted order to reconstruct the store.

`.xvc/ec/` holds the entity generator state (ensures globally unique `XvcEntity` values across runs).

### Adding a new pipeline dependency type

1. Create `pipeline/src/pipeline/deps/<name>.rs` with the dep struct, `persist!`, and `Storable`.
2. Add a variant to `XvcDependency` enum in `deps/mod.rs`.
3. Implement `compare_dependency` in `deps/compare.rs`.
4. Add CLI parsing in `pipeline/src/pipeline/api/` and wire into `cmd_step_dependency`.

### Recheck methods

`RecheckMethod` controls how files are placed in the workspace from cache: `Copy`, `Symlink`, `Hardlink`, or `Reflink` (feature-gated). Default is `Copy`.

### Documentation tests

`lib/tests/z_test_docs.rs` uses `trycmd` to run all shell commands embedded in the `book/src/` Markdown files. It symlinks docs into `lib/docs/` and template dirs from `lib/templates/`. These tests are macOS-only (`#[cfg(target_os = "macos")]`). Each doc page can have a `.in/` directory (input files) and `.out/` directory (expected outputs).

### Features

Default features enable all cloud backends. Key feature flags:
- `reflink` — enables reflink support via `xvc-file/reflink`
- `bundled-sqlite` — bundles SQLite for pipeline sqlite-query dependencies
- `test-ci` — enables all storage integration tests that require credentials
