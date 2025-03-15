# LLVM_PROFILE_FILE="${TMPDIR}/xvc-%p-%m.profraw" CARGO_INCREMENTAL=0 RUSTFLAGS="-Cinstrument-coverage" XVC_TRYCMD_TESTS=storage,file,pipeline,core,start TRYCMD=overwrite cargo llvm-cov --features test-ci --lcov --output-path lcov.info -p xvc # --test z_test_docs
# rws is for local dev, run-with-secrets script
# XVC_TRYCMD_TESTS=storage,file,pipeline,core,start TRYCMD=overwrite rws cargo test --features test-ci -p xvc

# XVC_TRYCMD_TESTS=storage,file,pipeline,core,start TRYCMD=overwrite cargo test --features test-ci -p xvc --test z_test_docs

XVC_TRYCMD_TESTS=storage TRYCMD=overwrite rws cargo test --features test-ci -p xvc

