#!/bin/sh
set -e
set -x
pwd
cd ..
pwd
rm target/llvm-cov-target/* || true
cargo llvm-cov clean --workspace
cargo llvm-cov --no-report --workspace --all-features -- --skip agent
cd tests
RUST_BACKTRACE=1 poetry run pytest -s $@
cargo llvm-cov --no-run --hide-instantiations --html
